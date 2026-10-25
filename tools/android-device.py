#!/usr/bin/env python3
"""Owned Android sessions and product-bound acceptance. Diagnostics remain diagnostic."""
from __future__ import annotations
import argparse
from dataclasses import dataclass
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import re
import shlex
import signal
import socket
import subprocess
import sys
import time
import uuid
import zipfile

REPO=Path(__file__).resolve().parent.parent
SCHEMA='femacs-android-device-v1'


def sha(path):
    with Path(path).open('rb') as file:
        digest=hashlib.sha256()
        for block in iter(lambda:file.read(65536),b''):digest.update(block)
        return digest.hexdigest()


def stamp():
    return datetime.now(timezone.utc).isoformat().replace('+00:00','Z')


def write_json(path,data):
    path=Path(path)
    candidate=path.with_name(path.name+'.next')
    with candidate.open('x') as file:
        json.dump(data,file,ensure_ascii=False,indent=2)
        file.write('\n')
    candidate.replace(path)


@dataclass(frozen=True)
class DeviceSpec:
    avd_name:str
    avd_home:Path
    serial:str
    port:int
    system_image:str
    api:int
    abi:str


@dataclass(frozen=True)
class Product:
    apk:Path
    so:Path
    apk_sha256:str
    so_sha256:str
    package:str


class DeviceSession:
    def __init__(self,spec,product,out,sdk_root,*,boot_timeout=300,launch_timeout=40,memory_mib=2048):
        self.spec=spec;self.product=product;self.out=Path(out).resolve();self.sdk=Path(sdk_root).resolve()
        self.boot_timeout=boot_timeout;self.launch_timeout=launch_timeout;self.memory_mib=memory_mib
        self.emulator=None;self.server=None;self.files=[];self.events=[];self.started=False;self.stopped=False;self.cleanup_result=None
        self.env=dict(os.environ);self.adb=self.sdk/'platform-tools/adb';self.native_dir=None;self.installed_apk=None
        self.started_at=stamp();self.start_clock=time.monotonic();self.pid=None;self.launched_at=None
        self.run_id=uuid.uuid4().hex;self.private_dir=None

    def command(self,args,*,input=None,timeout=60,check=True,max_output=64*1024*1024):
        started=time.monotonic();index=len(self.events)
        if not self.out.is_dir():raise RuntimeError('command output requires the owned run directory')
        output_path=self.out/f'command-{index}.stdout';error_path=self.out/f'command-{index}.stderr'
        event={'argv':[str(a) for a in args],'exit_code':None,
               'stdout':output_path.name,'stderr':error_path.name}
        try:
            with output_path.open('xb') as output,error_path.open('xb') as error:
                result=subprocess.run([str(a) for a in args],input=input,stdout=output,
                                      stderr=error,env=self.env,timeout=timeout)
            event['exit_code']=result.returncode
        except subprocess.TimeoutExpired:
            event['timed_out']=True
            raise
        finally:
            event['elapsed_ms']=(time.monotonic()-started)*1000
            self.events.append(event)
        # Stream to owned files first; oversized diagnostics never become an in-memory capture.
        if output_path.stat().st_size+error_path.stat().st_size>max_output:
            raise RuntimeError('oversized device command response; full streamed logs retained')
        output=output_path.read_bytes()
        if check and result.returncode:
            with error_path.open('rb') as error:
                error.seek(max(0,error_path.stat().st_size-2000))
                message=error.read(2000).decode(errors='replace')
            raise RuntimeError(f'device command failed ({result.returncode}): {message}')
        return output

    def adb_command(self,args,**kwargs):
        return self.command([self.adb,'-s',self.spec.serial,*args],**kwargs)

    def shell(self,args,*,input=None,timeout=60):
        # adb joins remote words. Quote every argument, including empty/literal words.
        return self.adb_command(['shell',' '.join(shlex.quote(str(a)) for a in args)],input=input,timeout=timeout)

    def start(self):
        if self.started:raise RuntimeError('owned device session already started')
        spec=self.spec;home=Path(spec.avd_home)
        if spec.serial!=f'emulator-{spec.port}' or spec.port%2 or not 1024<=spec.port<=65534:
            raise ValueError('serial and explicit even emulator port disagree')
        if not re.fullmatch(r'[A-Za-z0-9_-]+',spec.avd_name):raise ValueError('unsafe AVD name')
        if home.exists() or home.is_symlink() or not home.resolve().is_relative_to(self.out):
            raise ValueError('AVD home must be fresh and inside this owned run')
        image=self.sdk/Path(*spec.system_image.split(';'))
        properties=(image/'source.properties').read_text()
        if f'AndroidVersion.ApiLevel={spec.api}' not in properties or f'SystemImage.Abi={spec.abi}' not in properties:
            raise ValueError('installed system image metadata differs from requested identity')
        for program in [self.adb,self.sdk/'emulator/emulator',self.sdk/'cmdline-tools/latest/bin/avdmanager']:
            if not program.is_file():raise FileNotFoundError(program)
        # Reserve/check only our selected ports. A race fails startup, never kills a receiver.
        reservations=[]
        try:
            for port in (spec.port,spec.port+1):
                listener=socket.socket();listener.bind(('127.0.0.1',port));reservations.append(listener)
            self.out.mkdir(parents=True,exist_ok=True);home.mkdir(parents=True)
            self.env['ANDROID_AVD_HOME']=str(home);self.env['ANDROID_USER_HOME']=str(self.out/'android-user')
            self.command([self.sdk/'cmdline-tools/latest/bin/avdmanager','create','avd','-n',spec.avd_name,'-k',spec.system_image,'-p',home/(spec.avd_name+'.avd')],input=b'no\n',timeout=120)
            config=home/(spec.avd_name+'.avd')/'config.ini'
            if config.is_symlink() or not config.is_file() or not config.resolve().is_relative_to(home.resolve()):
                raise RuntimeError('owned AVD configuration missing or outside its home')
            # AVD configuration takes precedence over -partition-size defaults.
            # Request a bounded transient partition; the system image may raise
            # this to its own minimum. This is not an actual disk-usage limit.
            lines=[line for line in config.read_text().splitlines() if not line.startswith('disk.dataPartition.size=')]
            config.write_text('\n'.join(lines+['disk.dataPartition.size=2G'])+'\n')
            with socket.socket() as listener:
                listener.bind(('127.0.0.1',0));server_port=listener.getsockname()[1]
            self.env['ADB_SERVER_SOCKET']=f'tcp:127.0.0.1:{server_port}'
            # QEMU's host notification uses this port, independently of adb clients.
            self.env['ANDROID_ADB_SERVER_PORT']=str(server_port)
            server_log=(self.out/'adb-server.log').open('xb');self.files.append(server_log)
            # Server -L accepts a local tcp:PORT listener; clients may name its host.
            self.server=subprocess.Popen([str(self.adb),'-L',f'tcp:{server_port}','nodaemon','server'],env=self.env,stdin=subprocess.DEVNULL,stdout=server_log,stderr=subprocess.STDOUT)
            for listener in reservations:listener.close()
            reservations=[]
            emulator_log=(self.out/'emulator.log').open('xb');self.files.append(emulator_log)
            self.emulator=subprocess.Popen([str(self.sdk/'emulator/emulator'),'-avd',spec.avd_name,'-port',str(spec.port),'-no-window','-no-audio','-no-boot-anim','-no-snapshot','-memory',str(self.memory_mib),'-partition-size','2048','-gpu','swiftshader_indirect'],env=self.env,stdin=subprocess.DEVNULL,stdout=emulator_log,stderr=subprocess.STDOUT)
            self.started=True
            write_json(self.out/'ownership.json',{'run_id':self.run_id,'spec':{**spec.__dict__,'avd_home':str(home)},'guest_memory_mib':self.memory_mib,'requested_partition_mib':2048,'emulator_pid':self.emulator.pid,'server_pid':self.server.pid,'started_at':self.started_at})
        finally:
            for listener in reservations:listener.close()

    def wait_boot(self):
        deadline=time.monotonic()+self.boot_timeout
        while time.monotonic()<deadline:
            if self.emulator is None or self.emulator.poll() is not None:raise RuntimeError('owned emulator exited before boot')
            if self.server is None or self.server.poll() is not None:raise RuntimeError('owned adb server exited')
            try:
                if self.shell(['getprop','sys.boot_completed'],timeout=min(10,max(.01,deadline-time.monotonic()))).strip()==b'1':break
            except (RuntimeError,subprocess.TimeoutExpired):pass
            time.sleep(min(.05,max(0,deadline-time.monotonic())))
        else:raise TimeoutError('owned emulator boot deadline exceeded')
        for key,expected in [('ro.build.version.sdk',str(self.spec.api)),('ro.product.cpu.abi',self.spec.abi),('ro.boot.qemu.avd_name',self.spec.avd_name)]:
            actual=self.shell(['getprop',key]).decode().strip()
            if actual!=expected:raise RuntimeError(f'device identity mismatch: {key}={actual}, expected {expected}')
        return {'serial':self.spec.serial,'api':self.spec.api,'abi':self.spec.abi,'avd_name':self.spec.avd_name}

    def install(self):
        p=self.product
        if sha(p.apk)!=p.apk_sha256 or sha(p.so)!=p.so_sha256:raise RuntimeError('requested product changed before install')
        response=self.adb_command(['install','-r',p.apk],timeout=120)
        if b'Success' not in response:raise RuntimeError('install did not confirm success')
        paths=self.shell(['pm','path',p.package]).decode().splitlines()
        apk=next((row.removeprefix('package:') for row in paths if row.startswith('package:') and row.endswith('/base.apk')),None)
        if not apk:raise RuntimeError('installed base APK path missing')
        package=self.shell(['dumpsys','package',p.package]).decode()
        match=re.search(r'\bnativeLibraryDir=(\S+)',package)
        if match:
            native_dir=match.group(1)
        else:
            legacy=re.search(r'\blegacyNativeLibraryDir=(\S+)',package)
            primary=re.search(r'\bprimaryCpuAbi=(\S+)',package)
            if not legacy or not primary or primary.group(1)!=self.spec.abi or self.spec.abi!='arm64-v8a':
                raise RuntimeError('installed native directory or matching package ABI missing')
            # Modern package dumps name the root; ARM64 libraries are below arm64.
            # The subsequent actual pull and digest comparison remain mandatory.
            native_dir=legacy.group(1).rstrip('/')+'/arm64'
        self.installed_apk=apk;self.native_dir=native_dir
        self.adb_command(['pull',apk,self.out/'installed.apk'])
        self.adb_command(['pull',self.native_dir+'/libemacs.so',self.out/'installed-libemacs.so'])
        device_apk=sha(self.out/'installed.apk');device_so=sha(self.out/'installed-libemacs.so')
        if device_apk!=p.apk_sha256 or device_so!=p.so_sha256:raise RuntimeError('installed product hash differs from current APK/native input')
        return {'status':'ok','exit_code':0,'device_apk_sha256':device_apk,'device_so_sha256':device_so}

    def private_file(self,name,data):
        if not re.fullmatch(r'[A-Za-z0-9_.-]+',name):raise ValueError('unsafe private fixture name')
        if self.private_dir is None:
            directory=self.shell(['run-as',self.product.package,'pwd']).decode().strip()
            if not directory.startswith('/data/') or '\n' in directory:raise RuntimeError('unexpected app private directory')
            self.private_dir=directory+'/files/femacs-acceptance-'+self.run_id
            self.shell(['run-as',self.product.package,'mkdir','-p',self.private_dir])
        path=self.private_dir+'/'+name
        # The only shell syntax is a fixed redirection; both paths are separately quoted.
        script='cat > '+shlex.quote(path)
        self.shell(['run-as',self.product.package,'sh','-c',script],input=data)
        return path

    def launch(self,startup_args):
        if any(',' in arg for arg in startup_args):raise ValueError('activity arguments must be comma-free; transfer arbitrary Lisp in a file')
        response=self.shell(['am','start','-W','-n',self.product.package+'/.EmacsActivity','--esa',self.product.package+'.STARTUP_ARGUMENTS',','.join(startup_args)],timeout=self.launch_timeout)
        if b'Error:' in response or b'Exception' in response:raise RuntimeError('activity launch failed')
        deadline=time.monotonic()+self.launch_timeout
        while time.monotonic()<deadline:
            result=self.shell(['pidof',self.product.package]).decode().strip()
            if result and result.split()[0].isdigit():self.pid=int(result.split()[0]);self.launched_at=time.monotonic();return {'pid':self.pid,'status':'launched'}
            time.sleep(.2)
        raise TimeoutError('editor process did not appear')

    def capture(self,name):
        if not re.fullmatch(r'[A-Za-z0-9_-]+',name):raise ValueError('unsafe capture name')
        data=self.adb_command(['exec-out','screencap','-p'])
        if not data.startswith(b'\x89PNG\r\n\x1a\n'):raise RuntimeError('capture did not return a current PNG')
        path=self.out/(name+'.png');path.write_bytes(data);return path

    def current_ui(self,name):
        if not re.fullmatch(r'[A-Za-z0-9_-]+',name):raise ValueError('unsafe UI evidence name')
        remote='/data/local/tmp/femacs-'+self.run_id+'-'+name+'.xml'
        self.shell(['uiautomator','dump','--compressed',remote],timeout=30)
        data=self.shell(['cat',remote])
        if not data.strip():raise RuntimeError('current UI hierarchy missing')
        (self.out/(name+'.xml')).write_bytes(data)
        return data

    def observation(self,path,*,text=None,point=None,timeout=10):
        deadline=time.monotonic()+timeout;last=None
        while time.monotonic()<deadline:
            try:
                data=json.loads(self.shell(['run-as',self.product.package,'cat',path]))
                if data.get('run_token')!=self.run_id or data.get('error') or data.get('window_system')!='android' or data.get('pid')!=self.pid:raise RuntimeError('app observation identity/error mismatch')
                if data.get('selected_buffer')!='*Femacs Android Acceptance*':raise RuntimeError('acceptance editing buffer is not selected')
                if (text is None or data.get('text')==text) and (point is None or data.get('point')==point):return data
                last=data
            except (ValueError,RuntimeError) as error:last=str(error)
            time.sleep(.2)
        raise TimeoutError(f'current app observation did not match: {last}')

    def stop(self):
        if self.stopped:return {**self.cleanup_result,'already_stopped':True}
        errors=[]
        # emu kill is sent only to the child serial this session actually started.
        if self.started and self.emulator and self.emulator.poll() is None:
            try:self.adb_command(['emu','kill'],timeout=5)
            except Exception as error:errors.append(str(error))
        for child in (self.emulator,self.server):
            if child is None:continue
            try:
                if child.poll() is None:
                    try:child.wait(timeout=3)
                    except subprocess.TimeoutExpired:
                        child.terminate()
                        try:child.wait(timeout=3)
                        except subprocess.TimeoutExpired:child.kill();child.wait(timeout=3)
            except Exception as error:errors.append(str(error))
        for file in self.files:file.close()
        self.stopped=True
        if self.out.is_dir():write_json(self.out/'commands.json',self.events)
        reaped=all(child is None or child.poll() is not None for child in (self.emulator,self.server))
        self.cleanup_result={'complete':reaped and not errors,'errors':errors,'owned_children_reaped':reaped}
        return self.cleanup_result


def load_receipt(path):
    with Path(path).open() as file:
        value=json.load(file)
    if not isinstance(value,dict):raise ValueError('receipt must be an object')
    return value


def pattern_colors(token):
    """Five reproducible per-run display-only colors, kept away from UI grays."""
    colors=[]
    for index in range(5):
        salt=0
        while True:
            value=hashlib.sha256(f'{token}:cell:{index}:{salt}'.encode()).digest()
            rgb=tuple(32+int(channel)*191//255 for channel in value[:3])
            if max(rgb)-min(rgb)>=50 and all(sum(abs(a-b) for a,b in zip(rgb,other))>=80 for other in colors):break
            salt+=1
        colors.append(rgb)
    return ['#%02x%02x%02x'%rgb for rgb in colors]


def is_observation(value,token,pid,point=None):
    return (isinstance(value,dict) and value.get('schema')=='femacs-android-observation-v1'
            and value.get('run_token')==token and value.get('pid')==pid
            and value.get('error') is None and value.get('text')=='HELLO'
            and value.get('modified') is True and value.get('window_system')=='android'
            and value.get('buffer')=='*Femacs Android Acceptance*'
            and value.get('selected_buffer')==value.get('buffer')
            and type(value.get('point')) is int and 1<=value['point']<=6
            and (point is None or value['point']==point))


def valid_render(value,token,pid,*,baseline=False):
    if not isinstance(value,dict) or not is_observation(value.get('observation'),token,pid):return False
    if value.get('pattern_validated') is not True or value.get('pattern_token')!=token or value.get('expected_text')!='HELLO':return False
    if not re.fullmatch('[0-9a-f]{64}',value.get('sha256','')):return False
    edges=value.get('editor_region');cells=value.get('pattern')
    if not isinstance(edges,list) or len(edges)!=4 or not all(type(n) is int for n in edges) or not(edges[0]<edges[2] and edges[1]<edges[3]):return False
    if not isinstance(cells,list) or len(cells)!=5:return False
    for cell,color in zip(cells,pattern_colors(token)):
        if not isinstance(cell,dict) or cell.get('color')!=color:return False
        rect=cell.get('rect')
        if not isinstance(rect,list) or len(rect)!=4 or not all(type(n) is int for n in rect):return False
        x,y,r,b=rect
        if not (edges[0]<=x<r<=edges[2] and edges[1]<=y<b<=edges[3]):return False
        if type(cell.get('matching_pixels')) is not int or type(cell.get('total_pixels')) is not int:return False
        if cell['total_pixels']!=(r-x)*(b-y) or not max(4,.15*cell['total_pixels'])<=cell['matching_pixels']<=cell['total_pixels']:return False
    if baseline and (not re.fullmatch('[0-9a-f]{64}',value.get('before_sha256','')) or value.get('before_sha256')==value['sha256'] or value.get('changed_pixels',0)<=2*value['observation'].get('char_height',0)):return False
    return True


def _judge(receipt,apk,so):
    errors=[]
    if receipt.get('schema')!=SCHEMA:errors.append('receipt schema missing')
    mode=receipt.get('mode');strict=mode in ('accept','smoke')
    if not strict or receipt.get('acceptance') is not True:errors.append('diagnostic/incomplete receipt is not readiness')
    product=receipt.get('product',{})
    for key,path in [('apk',apk),('so',so)]:
        expected=sha(path)
        if product.get(key+'_sha256')!=expected or product.get('device_'+key+'_sha256')!=expected:errors.append('current/installed '+key+' identity mismatch')
    if product.get('package')!='org.femacs':errors.append('wrong package')
    avd=receipt.get('owned_avd',{});port=receipt.get('port');token=receipt.get('run_id')
    if not isinstance(token,str) or not token or type(port) is not int or not 5554<=port<=5682 or port%2 or receipt.get('serial')!=f'emulator-{port}' or not avd.get('name') or avd.get('api')!=36 or avd.get('abi')!='arm64-v8a' or avd.get('system_image')!='system-images;android-36;default;arm64-v8a':errors.append('owned device identity missing/inconsistent')
    root=Path(receipt['run_root']);home=Path(avd['home'])
    # Fresh owner creates these under build/cache. A receipt cannot substitute a
    # foreign AVD merely by putting a nonempty path into its ownership field.
    relative=root.resolve().relative_to(REPO.resolve())
    if len(relative.parts)<2 or relative.parts[0] not in ('build','.femacs-cache') or root!=root.resolve() or home!=root/'avd' or home!=home.resolve():errors.append('AVD home escapes exact owned run root')
    if not root.is_dir() or not home.is_dir():errors.append('owned run/AVD directory missing')
    ownership_path=root/'ownership.json'
    if not ownership_path.is_file() or ownership_path.is_symlink() or ownership_path.stat().st_size>65536:
        errors.append('physical ownership record missing/invalid')
    else:
        ownership=load_receipt(ownership_path)
        expected_spec={'avd_name':avd.get('name'),'avd_home':str(home),'serial':receipt.get('serial'),
                       'port':port,'system_image':avd.get('system_image'),'api':avd.get('api'),'abi':avd.get('abi')}
        if ownership.get('run_id')!=token or ownership.get('spec')!=expected_spec or ownership.get('guest_memory_mib')!=avd.get('memory_mib',2048) or any(type(ownership.get(key)) is not int or ownership[key]<=0 for key in ('emulator_pid','server_pid')):
            errors.append('physical ownership record differs from run/device identity')
    if receipt.get('install',{}).get('status')!='ok' or receipt.get('install',{}).get('exit_code')!=0:errors.append('install not successful')
    process=receipt.get('process',{});native=receipt.get('native',{});pid=process.get('pid')
    if type(pid) is not int or pid<=0 or type(process.get('alive_seconds')) not in (int,float) or process['alive_seconds']<20:errors.append('current process survival missing')
    if native.get('init_line') is not True or native.get('thread_line') is not True or type(native.get('stderr_lines')) is not int or native['stderr_lines']<1:errors.append('native initialization evidence missing')
    if receipt.get('crashes',{}).get('fatal_signals')!=0:errors.append('crash evidence present/missing')
    if receipt.get('errors')!=[]:errors.append('run contains errors or missing error record')
    needed={'install','identity','native','liveness'}
    if mode=='accept':needed|={'render','input','keys','clipboard','battery','rotation'}
    cases=receipt.get('cases',[]);by_id={}
    for case in cases:
        name=case['id']
        if name in by_id:errors.append('duplicate case '+name)
        by_id[name]=case.get('evidence')
        if case.get('status')!='passed' or case.get('kind')!='strict' or type(case.get('elapsed_ms')) not in (int,float) or case['elapsed_ms']<0:errors.append('failed/incomplete requested case '+name)
    if not needed<=by_id.keys():errors.append('required current cases missing: '+','.join(sorted(needed-by_id.keys())))
    identity=by_id.get('identity',{})
    if identity!={'serial':receipt.get('serial'),'api':avd.get('api'),'abi':avd.get('abi'),'avd_name':avd.get('name')}:errors.append('identity evidence mismatch')
    install=by_id.get('install',{})
    if install!=receipt.get('install') or any(install.get('device_'+k+'_sha256')!=product.get(k+'_sha256') for k in ('apk','so')):errors.append('installed-product case evidence missing/mismatched')
    if by_id.get('native')!=native or by_id.get('liveness')!=process:errors.append('native/liveness case evidence missing/mismatched')
    if mode=='accept':
        if not is_observation(by_id.get('input'),token,pid,6):errors.append('current input evidence missing/mismatched')
        keys=by_id.get('keys')
        if not isinstance(keys,list) or len(keys)!=4 or not all(is_observation(v,token,pid,point) for v,point in zip(keys or [],(1,6,5,6))):errors.append('key movement evidence missing/mismatched')
        clip=by_id.get('clipboard',{})
        if not is_observation(clip,token,pid,6) or clip.get('clipboard_actual')!='Femacs clipboard 中文 '+token or clip.get('clipboard_expected')!=clip.get('clipboard_actual'):errors.append('clipboard evidence missing/mismatched')
        battery_observation=by_id.get('battery',{})
        if not is_observation(battery_observation,token,pid,6):errors.append('battery observation identity mismatch')
        battery=battery_observation.get('battery')
        if not isinstance(battery,list) or len(battery)!=8 or not all(type(v) is int for v in battery) or not 0<=battery[0]<=100 or battery[4] not in (1,2,3,4,5):errors.append('battery native result missing/malformed')
        if not valid_render(by_id.get('render'),token,pid,baseline=True):errors.append('current token-pattern rendering evidence missing/mismatched')
        rotations=by_id.get('rotation')
        if not isinstance(rotations,list) or len(rotations)!=2:errors.append('rotation evidence missing')
        else:
            for entry,rotation in zip(rotations,(1,0)):
                observation=entry.get('observation',{})
                if entry.get('requested_rotation')!=rotation or not valid_render(entry,token,pid) or (observation.get('frame_width',0)>observation.get('frame_height',0))!=(rotation==1):errors.append('rotation content/geometry evidence mismatch')
    cleanup=receipt.get('cleanup',{})
    if cleanup.get('complete') is not True or cleanup.get('owned_children_reaped') is not True or cleanup.get('errors')!=[]:errors.append('owned cleanup incomplete/errors')
    duration=(datetime.fromisoformat(receipt['completed_at'].replace('Z','+00:00'))-datetime.fromisoformat(receipt['started_at'].replace('Z','+00:00'))).total_seconds()
    if duration<process.get('alive_seconds',0) or duration<20:errors.append('inconsistent observation duration')
    return {'passed':not errors,'acceptance':not errors,'scope':'editing' if mode=='accept' else 'startup','errors':errors}


def judge(receipt,apk,so):
    try:return _judge(receipt,apk,so)
    except (KeyError,TypeError,ValueError,AttributeError,OSError) as error:
        return {'passed':False,'acceptance':False,'scope':'invalid','errors':['invalid/incomplete evidence: '+str(error)]}


def free_port():
    for port in range(5554,5683,2):
        sockets=[]
        try:
            for number in (port,port+1):
                sock=socket.socket();sock.bind(('127.0.0.1',number));sockets.append(sock)
            return port
        except OSError:pass
        finally:
            for sock in sockets:sock.close()
    raise RuntimeError('no free emulator port pair')


def content_region(observation,ui_xml,image_size):
    import xml.etree.ElementTree as ET
    width,height=image_size
    frame_width=int(observation['frame_width']);frame_height=int(observation['frame_height'])
    candidates=set()
    for node in ET.fromstring(ui_xml).iter('node'):
        if node.get('package')!='org.femacs':continue
        match=re.fullmatch(r'\[(\d+),(\d+)\]\[(\d+),(\d+)\]',node.get('bounds',''))
        if not match:continue
        x,y,right,bottom=map(int,match.groups())
        if right-x==frame_width and bottom-y==frame_height and 0<=x<right<=width and 0<=y<bottom<=height:
            candidates.add((x,y,right,bottom))
    if len(candidates)!=1:raise RuntimeError('current app/client geometry missing or ambiguous')
    x,y,right,bottom=next(iter(candidates))
    edges=observation.get('window_edges')
    if not isinstance(edges,list) or len(edges)!=4:raise RuntimeError('current app editor edges missing')
    left,top,r,b=map(int,edges)
    if not (0<=left<r<=frame_width and 0<=top<b<=frame_height):raise RuntimeError('editor region outside actual app client')
    return [x+left,y+top,x+r,y+b]


def coherent_capture(session,observe,name):
    """Keep strict client geometry while startup resize is still being applied."""
    from PIL import Image
    attempts=[]
    last=None
    for index in range(3):
        capture_name=name if index==0 else name+'-'+str(index+1)
        entry={'image':capture_name+'.png',
               'ui':capture_name+'-ui.xml','passed':False}
        try:
            before=observe();entry['before']=before
            image=session.capture(capture_name);entry['image']=image.name
            ui_xml=session.current_ui(capture_name+'-ui')
            after=observe();entry['after']=after
            try:
                keys=('frame_width','frame_height','window_edges')
                if any(before.get(key)!=after.get(key) for key in keys):
                    raise RuntimeError('editor geometry changed during the current capture')
                with Image.open(image) as pixels:
                    pixels.load()
                    content_region(after,ui_xml,pixels.size)
                entry['passed']=True
            except RuntimeError as error:
                last=str(error);entry['error']=last
        except Exception as error:
            entry.update(error=str(error),error_type=type(error).__name__)
            attempts.append(entry)
            write_json(session.out/(name+'-geometry.json'),{'attempts':attempts})
            raise
        attempts.append(entry)
        write_json(session.out/(name+'-geometry.json'),{'attempts':attempts})
        if entry['passed']:
            return after,image,ui_xml
    raise RuntimeError('coherent client geometry unavailable after three snapshots: '+str(last))


def key_movements(session,observe):
    observations=[];attempts=[]
    for argv,point in [(['input','keycombination','113','29'],1),
                       (['input','keycombination','113','33'],6),
                       (['input','keyevent','21'],5),(['input','keyevent','22'],6)]:
        started=time.monotonic();entry={'argv':argv,'expected_point':point,'passed':False}
        try:
            session.shell(argv)
            value=observe(text='HELLO',point=point)
            observations.append(value);entry.update(passed=True,observation=value)
        except Exception as error:
            entry.update(error=str(error),error_type=type(error).__name__)
            raise
        finally:
            entry['elapsed_seconds']=time.monotonic()-started
            attempts.append(entry)
            write_json(session.out/'key-movement-observations.json',{'attempts':attempts})
    return observations


def rendering(path,observation,*,ui_xml,before=None):
    from PIL import Image,ImageChops
    if observation.get('text')!='HELLO' or observation.get('modified') is not True:raise RuntimeError('render requires actual current text input')
    token=observation['run_token'];cells=observation.get('display_pattern')
    if not isinstance(cells,list) or len(cells)!=5:raise RuntimeError('current glyph-cell observations missing')
    with Image.open(path) as image:
        image.load();image=image.convert('RGB');edges=content_region(observation,ui_xml,image.size)
        origin=[edges[0]-observation['window_edges'][0],edges[1]-observation['window_edges'][1]]
        measured=[]
        for cell,color in zip(cells,pattern_colors(token)):
            if cell.get('color')!=color:raise RuntimeError('glyph pattern is not bound to current run token')
            rect=cell['rect']
            if len(rect)!=4 or not all(type(n) is int for n in rect):raise RuntimeError('invalid observed glyph rectangle')
            x,y,r,b=[v+origin[i%2] for i,v in enumerate(rect)]
            if not(edges[0]<=x<r<=edges[2] and edges[1]<=y<b<=edges[3]):raise RuntimeError('glyph pattern outside actual editor region')
            pixels=image.crop((x,y,r,b));expected=tuple(bytes.fromhex(color[1:]));total=pixels.width*pixels.height
            matching=sum(all(abs(a-b)<=10 for a,b in zip(pixel,expected)) for pixel in pixels.getdata())
            if matching<max(4,.15*total):raise RuntimeError('current character-cell pattern not rendered')
            measured.append({'color':color,'rect':[x,y,r,b],'matching_pixels':matching,'total_pixels':total})
        evidence={'image':[image.width,image.height],'editor_region':edges,'sha256':sha(path),
                  'observation':observation,'pattern_validated':True,'pattern_token':token,'expected_text':'HELLO','pattern':measured}
        if before is not None:
            with Image.open(before) as previous:
                previous.load()
                if previous.size!=image.size:raise RuntimeError('baseline/current render dimensions differ')
                difference=ImageChops.difference(image.crop(edges),previous.crop(edges).convert('RGB'))
                changed=sum(any(pixel) for pixel in difference.getdata())
                if changed<=2*int(observation['char_height']):raise RuntimeError('no sufficient current editor-content change')
                evidence.update(before_sha256=sha(before),changed_pixels=changed)
        return evidence


def failure_diagnostics(session,out):
    """Retain bounded read-only evidence before stopping this owned receiver."""
    results=[]
    requests=[('failure-main.log',['logcat','-d','-t','512','-v','threadtime']),
              ('failure-crash.log',['logcat','-b','crash','-d','-t','128']),
              ('failure-activity.log',['shell','dumpsys activity activities']),
              ('failure-window.log',['shell','dumpsys window windows'])]
    if type(session.pid) is int and session.pid>0:
        # The general ring's tail can contain only unrelated busy-system logs.
        requests.insert(0,('failure-app.log',['logcat','-d','--pid='+str(session.pid),'-t','512','-v','threadtime']))
    for name,argv in requests:
        try:
            data=session.adb_command(argv,timeout=10,max_output=4*1024*1024)
            (out/name).write_bytes(data);results.append({'file':name,'status':'captured'})
        except Exception as error:results.append({'file':name,'status':'unavailable','error':str(error)})
    try:
        path=session.capture('failure-screen');results.append({'file':path.name,'status':'captured'})
    except Exception as error:results.append({'file':'failure-screen.png','status':'unavailable','error':str(error)})
    return {'kind':'diagnostic','acceptance':False,'entries':results}


def native_fault_count(data):
    """Count fatal log markers, including CheckJNI before its eventual abort."""
    return len(re.findall(rb'Fatal signal|FATAL EXCEPTION|Abort message|dlopen failed|cannot locate symbol|JNI DETECTED ERROR IN APPLICATION',data))


def run_owned(args,out):
    apk=args.apk.resolve();so=args.so.resolve();product=Product(apk,so,sha(apk),sha(so),'org.femacs')
    with zipfile.ZipFile(apk) as archive:
        if hashlib.sha256(archive.read('lib/arm64-v8a/libemacs.so')).hexdigest()!=product.so_sha256:raise RuntimeError('APK does not contain the requested native library')
    port=args.port or free_port();run_id=uuid.uuid4().hex
    spec=DeviceSpec(args.avd+'-'+run_id[:10],out/'avd',f'emulator-{port}',port,'system-images;android-36;default;arm64-v8a',36,'arm64-v8a')
    session=DeviceSession(spec,product,out,args.sdk,boot_timeout=args.boot_timeout,launch_timeout=args.launch_timeout,memory_mib=args.memory)
    receipt={'schema':SCHEMA,'run_id':session.run_id,'run_root':str(out.resolve()),'mode':args.mode,'acceptance':False,'owned_avd':{'name':spec.avd_name,'home':str(spec.avd_home),'memory_mib':args.memory,'system_image':spec.system_image,'api':spec.api,'abi':spec.abi},'serial':spec.serial,'port':port,'product':{'apk_sha256':product.apk_sha256,'so_sha256':product.so_sha256,'package':product.package},'cases':[],'errors':[],'started_at':stamp()}
    def case(name,action):
        started=time.monotonic()
        try:
            value=action();receipt['cases'].append({'id':name,'kind':'strict' if args.mode in ('accept','smoke') else 'diagnostic','status':'passed','evidence':value,'elapsed_ms':(time.monotonic()-started)*1000});return value
        except Exception as error:
            receipt['cases'].append({'id':name,'status':'failed','error':str(error),'elapsed_ms':(time.monotonic()-started)*1000});raise
    try:
        session.start();case('identity',session.wait_boot);install=case('install',session.install);receipt['install']=install;receipt['product'].update({key:install[key] for key in ('device_apk_sha256','device_so_sha256')})
        log_since=session.shell(['date','+%m-%d %H:%M:%S.000']).decode().strip()
        if not re.fullmatch(r'\d\d-\d\d \d\d:\d\d:\d\d\.\d{3}',log_since):raise RuntimeError('device log timestamp unavailable')
        if args.mode=='batch':
            body=args.elisp_file.read_bytes() if args.elisp_file else ('(progn (princ (format "%S\\n" '+args.expression+')) (kill-emacs 0))\n').encode()
            fixture=session.private_file('batch.el',body)
            output=session.shell(['run-as',product.package,'env','LD_LIBRARY_PATH='+session.native_dir,'CLASSPATH='+session.installed_apk,'app_process','/system/bin','org.femacs.EmacsNoninteractive','libandroid-emacs.so','--batch','-l',fixture],timeout=max(args.wait,args.launch_timeout))
            (out/'batch.stdout').write_bytes(output);print(output.decode(errors='replace'),end='');case('batch',lambda:{'stdout':'batch.stdout'});receipt['result']=output.decode(errors='replace')
        else:
            if args.mode=='accept':
                # Define private paths before the fixture is transferred. Arbitrary Lisp stays in files.
                session.private_file('anchor',b'owned\n');report=session.private_dir+'/observation.json'
                colors_lisp='('+' '.join(json.dumps(color) for color in pattern_colors(session.run_id))+')'
                prefix=f"(setq femacs-android-report-file {json.dumps(report)} femacs-android-run-token {json.dumps(session.run_id)} femacs-android-pattern-colors '{colors_lisp})\n"
                body=b';;; Owned device fixture -*- lexical-binding: t; -*-\n'+prefix.encode()+ (REPO/'tools/fixtures/android-acceptance.el').read_bytes()
                fixture=session.private_file('acceptance.el',body);startup=['--load',fixture]
            elif args.mode in ('gui','probe'):
                body=args.elisp_file.read_bytes() if args.elisp_file else (args.expression+'\n').encode();fixture=session.private_file('diagnostic.el',body);startup=['--load',fixture]
            else:startup=[]
            case('launch',lambda:session.launch(startup))
            if args.mode=='accept':
                observe=lambda **kwargs:session.observation(report,**kwargs)
                initial,empty_capture,initial_ui=coherent_capture(
                    session,lambda:observe(text='',point=1),'empty-editor')
                case('input',lambda:(session.shell(['input','text','HELLO']),observe(text='HELLO',point=6))[-1])
                case('keys',lambda:key_movements(session,observe))
                def clipboard():
                    value=observe(text='HELLO',point=6)
                    if value.get('clipboard_actual')!=value.get('clipboard_expected') or not value.get('clipboard_actual'):raise RuntimeError('clipboard round-trip mismatch')
                    return value
                case('clipboard',clipboard)
                def battery():
                    value=observe(text='HELLO',point=6)
                    if not isinstance(value.get('battery'),list) or len(value['battery'])!=8 or not all(type(v) is int for v in value['battery']):raise RuntimeError('battery native observation missing')
                    return value
                case('battery',battery)
                def render_current():
                    value=observe(text='HELLO',point=6)
                    return rendering(session.capture('editor'),value,ui_xml=session.current_ui('editor-ui'),before=empty_capture)
                case('render',render_current)
                def rotation():
                    original=session.shell(['settings','get','system','accelerometer_rotation']).decode().strip();angle=session.shell(['settings','get','system','user_rotation']).decode().strip();observations=[]
                    try:
                        session.shell(['settings','put','system','accelerometer_rotation','0'])
                        for rotation in (1,0):
                            session.shell(['settings','put','system','user_rotation',str(rotation)]);time.sleep(1)
                            value=observe(text='HELLO')
                            if (value['frame_width']>value['frame_height'])!=(rotation==1):raise RuntimeError('requested frame rotation not observed')
                            rendered=dict(rendering(session.capture('rotation-'+str(rotation)),value,ui_xml=session.current_ui('rotation-ui-'+str(rotation))),requested_rotation=rotation)
                            observations.append(rendered)
                            session.shell(['input','tap',str(rendered['editor_region'][0]+8),str(rendered['editor_region'][1]+8)])
                            # Tap may legitimately move point; preserve exact text and selected buffer.
                            observe(text='HELLO')
                    finally:
                        session.shell(['settings','put','system','user_rotation',angle]);session.shell(['settings','put','system','accelerometer_rotation',original])
                    return observations
                case('rotation',rotation)
            started=session.launched_at
            while time.monotonic()-started<max(20,args.wait if args.mode in ('gui','probe') else 20):
                current=session.shell(['pidof',product.package]).decode().split()
                if str(session.pid) not in current:raise RuntimeError('editor died during survival observation')
                time.sleep(.5)
            receipt['process']={'pid':session.pid,'alive_seconds':time.monotonic()-started};case('liveness',lambda:receipt['process'])
            log=session.adb_command(['logcat','-d','-v','threadtime','--pid='+str(session.pid),'-T',log_since]);crash=session.adb_command(['logcat','-b','crash','-d','-T',log_since]);(out/'main.log').write_bytes(log);(out/'crash.log').write_bytes(crash)
            session.shell(['dumpsys','activity','exit-info',product.package])
            native={'init_line':b'Initializing ' in log and re.search(rb'EmacsThread.*libandroid-emacs\.so',log) is not None,'thread_line':b'android_run_debug_thread' in log,'stderr_lines':sum(b'android_run_debug_thread' in row for row in log.splitlines()),'scope':'Java params closure returned; launcher call and native stdout bridge observed'};receipt['native']=native
            fatal=native_fault_count(log+crash);receipt['crashes']={'fatal_signals':fatal,'scope':'fatal log markers, not distinct crash count'}
            def native_case():
                if fatal or not native['init_line'] or not native['thread_line']:raise RuntimeError('current native initialization missing or fatal log present')
                return native
            case('native',native_case)
        receipt['acceptance']=args.mode in ('accept','smoke')
    except Exception as error:
        receipt['errors'].append(str(error))
        if session.started:receipt['failure_diagnostics']=failure_diagnostics(session,out)
    finally:
        receipt['cleanup']=session.stop();receipt['completed_at']=stamp();write_json(out/'receipt.json',receipt)
        tail_owned_logs(out,args.tail)
    if args.mode in ('accept','smoke'):
        result=judge(receipt,apk,so);write_json(out/'readiness.json',result);return 0 if result['passed'] else 1
    return 1 if receipt['errors'] else 0


def tail_owned_logs(out,lines):
    if lines is None or lines==0:return
    from collections import deque
    for path in sorted([*out.glob('diagnostic-*.log'),*[out/name for name in ('main.log','crash.log') if (out/name).is_file()]]):
        with path.open(errors='replace') as source:
            print('== '+path.name+' ==')
            for line in deque(source,maxlen=lines):print(line,end='')


class DeviceParser(argparse.ArgumentParser):
    def error(self,message):
        super().error(message+'; migration: docs/repository-bootstrap.md#android-device-command-migration')


def main(argv=None):
    parser=DeviceParser(description=__doc__,epilog='Legacy diagnostic stages and parameter migration: docs/repository-bootstrap.md#android-device-command-migration')
    parser.add_argument('mode',choices=('inspect','dump','smoke','accept','batch','gui','probe'))
    parser.add_argument('--serial');parser.add_argument('--port',type=int)
    parser.add_argument('--apk',type=Path);parser.add_argument('--so',type=Path)
    parser.add_argument('--sdk',type=Path,default=Path(os.environ.get('ANDROID_HOME',str(Path.home()/'Library/Android/sdk'))))
    parser.add_argument('--out',type=Path);parser.add_argument('--avd',default='femacs-owned')
    parser.add_argument('--boot-timeout',type=float,default=300);parser.add_argument('--launch-timeout',type=float,default=40)
    parser.add_argument('--wait',type=float,default=20)
    parser.add_argument('--eval',dest='expression');parser.add_argument('--elisp-file',type=Path)
    parser.add_argument('--plan',action='store_true',help='print the owned lifecycle without executing or creating output')
    parser.add_argument('--dump-only',action='store_true',help='legacy spelling for read-only dump; requires --serial')
    parser.add_argument('--memory',type=int,default=2048,help='owned emulator guest MiB; external controller limits still apply')
    parser.add_argument('--tail',type=int,help='print up to this many trailing lines per owned log (0..10000)')
    args=parser.parse_args(argv)
    if args.memory<=0 or args.tail is not None and not 0<=args.tail<=10000:
        print('memory must be positive and tail must be0..10000',file=sys.stderr);return 2
    if args.dump_only:
        if args.mode not in ('smoke','dump'):
            print('--dump-only belongs to the legacy device-run/smoke or dump route',file=sys.stderr);return 2
        args.mode='dump'
    if args.plan:
        print(json.dumps({'schema':'femacs-android-device-plan-v1','executed':False,'mode':args.mode,
                         'receiver':args.serial if args.mode in ('inspect','dump') else 'fresh run-owned AVD and isolated adb',
                         'guest_memory_mib':args.memory,'steps':['validate product and image','create fresh owned run','boot and verify explicit serial','checked install and compare installed hashes','launch/observe requested scope','reap only owned children'] if args.mode not in ('inspect','dump') else ['read explicit receiver logs/state only']},ensure_ascii=False));return 0
    if args.mode in ('inspect','dump') and not args.serial:
        print('inspection requires explicit --serial',file=sys.stderr);return 2
    if args.mode not in ('inspect','dump') and (not args.apk or not args.so):
        print('owned launch requires explicit current --apk and --so',file=sys.stderr);return 2
    if args.mode in ('batch','gui','probe') and not(args.expression or args.elisp_file):
        print('diagnostic needs --eval or --elisp-file',file=sys.stderr);return 2
    try:
        out=(args.out or REPO/'build/acceptance/android-device'/uuid.uuid4().hex).resolve()
        if not (out.is_relative_to((REPO/'build').resolve()) or out.is_relative_to((REPO/'.femacs-cache').resolve())) or out.exists():raise ValueError('output must be a fresh owned build/cache directory')
        out.mkdir(parents=True)
        if args.mode in ('inspect','dump'):
            rows=[]
            for words in [['shell','getprop'],['logcat','-d'],['logcat','-b','crash','-d'],['shell','dumpsys','activity','exit-info','org.femacs'],['shell','dumpsys','window'],['shell','dumpsys','input'],['shell','dumpsys','activity','top'],['shell','dumpsys','input_method']]:
                with (out/f'diagnostic-{len(rows)}.log').open('xb') as log:
                    result=subprocess.run([str(args.sdk/'platform-tools/adb'),'-s',args.serial,*words],stdout=log,stderr=subprocess.STDOUT,timeout=60)
                rows.append({'argv':words,'exit_code':result.returncode})
            write_json(out/'receipt.json',{'schema':SCHEMA,'mode':args.mode,'serial':args.serial,'acceptance':False,'commands':rows})
            tail_owned_logs(out,args.tail)
            return 0 if all(row['exit_code']==0 for row in rows) else 1
        return run_owned(args,out)
    except Exception as error:
        if 'out' in locals() and out.is_dir() and not (out/'receipt.json').exists():
            write_json(out/'receipt.json',{'schema':SCHEMA,'mode':args.mode,'acceptance':False,'errors':[str(error)],'completed_at':stamp()})
        print(str(error),file=sys.stderr);return 2


if __name__=='__main__':
    for sig in (signal.SIGTERM,signal.SIGINT):
        signal.signal(sig,lambda signum,frame:(_ for _ in ()).throw(InterruptedError(f'interrupted by signal {signum}')))
    raise SystemExit(main())
