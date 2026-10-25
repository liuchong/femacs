#!/usr/bin/env python3
"""B-phase G4 failure/ownership/staging contracts, executed only after full C.

Fake SDK/editor/Texinfo executables exercise the actual argv and subprocess
boundaries. They never attach to an installed SDK or a user's device. Real
compiler/device coverage belongs to g4-acceptance.py, not these fake tools.
"""
from __future__ import annotations
import contextlib
import copy
import hashlib
import importlib.util
import io
import json
import re
import os
from pathlib import Path
import shlex
import shutil
import socket
import subprocess
import sys
import tempfile
import time
import unittest
from unittest import mock
import zipfile

REPO = Path(__file__).resolve().parent.parent


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def executable(path, body):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text('#!' + sys.executable + '\n' + body, encoding='utf-8')
    path.chmod(0o700)
    return path


SDK_FAKE = r'''
import json,os,shlex,shutil,sys,time
from pathlib import Path
state_path=Path(STATE_PATH)
state=json.loads(state_path.read_text())
root=state_path.parent
name=Path(sys.argv[0]).name
args=sys.argv[1:]
with (root/'events.jsonl').open('a') as log:
    log.write(json.dumps({'name':name,'args':args,'pid':os.getpid()})+'\n')
if name=='emulator':
    notify_port=os.environ.get('ANDROID_ADB_SERVER_PORT','')
    if not notify_port.isdigit() or not os.environ.get('ADB_SERVER_SOCKET','').endswith(':'+notify_port):
        print('emulator notification is not bound to the owned adb server',file=sys.stderr);sys.exit(94)
    (root/'emulator.pid').write_text(str(os.getpid()))
    while not (root/'stop-emulator').exists(): time.sleep(.02)
    sys.exit(0)
if name=='avdmanager':
    output=Path(args[args.index('-p')+1]);output.mkdir(parents=True,exist_ok=True)
    (output/'config.ini').write_text('disk.dataPartition.size=6G\n')
    sys.exit(0)
if 'nodaemon' in args and 'server' in args:
    listener=args[args.index('-L')+1].split(':')
    if len(listener)!=2 or listener[0]!='tcp' or not listener[1].isdigit():
        print('listening on specified hostname currently unsupported',file=sys.stderr);sys.exit(93)
    (root/'server.pid').write_text(str(os.getpid()))
    while True: time.sleep(.02)
if args==['version']:
    print('Android Debug Bridge version 1.0.41');sys.exit(0)
if '-s' not in args:
    print('fake SDK rejects an unselected device',file=sys.stderr);sys.exit(91)
at=args.index('-s');serial=args[at+1];args=args[at+2:]
if serial!=state['serial']:
    print('foreign serial',file=sys.stderr);sys.exit(92)
if args[:2]==['emu','kill']:
    (root/'stop-emulator').write_text('owned stop');sys.exit(0)
if args[:1]==['get-state']:
    print('device');sys.exit(0)
if args[:1]==['install']:
    if state.get('install_fail'):
        print('INSTALL_FAILED_TEST',file=sys.stderr);sys.exit(1)
    (root/'installed').write_text('yes');print('Success');sys.exit(0)
if args[:1]==['pull']:
    source,destination=args[-2:]
    if source.endswith('/libemacs.so') and source!=str(root/'installed/lib/arm64/libemacs.so'):
        print('native library absent at requested installed path',file=sys.stderr);sys.exit(95)
    original=Path(state['apk'] if source.endswith('.apk') else state['so'])
    if state.get('stale_product'):
        Path(destination).write_bytes(b'wrong installed product')
    else: shutil.copyfile(original,destination)
    print('1 file pulled');sys.exit(0)
if args[:1]==['shell']:
    words=shlex.split(' '.join(args[1:]))
    if words[:1]==['getprop']:
        prop=words[1]
        print({'sys.boot_completed':'0' if state.get('boot_fail') else '1',
               'ro.build.version.sdk':'36',
               'ro.product.cpu.abi':'x86_64' if state.get('wrong_abi') else 'arm64-v8a',
               'ro.boot.qemu.avd_name':state['avd_name']}.get(prop,''))
    elif words[:2]==['pm','path']:
        print('package:'+str(root/'installed/base.apk'))
    elif words[:2]==['dumpsys','package']:
        if state.get('legacy_native_layout'):
            print('legacyNativeLibraryDir='+str(root/'installed/lib'))
            print('primaryCpuAbi=arm64-v8a')
        else:print('nativeLibraryDir='+str(root/'installed/lib/arm64'))
    elif words[:1]==['printf']:
        print(json.dumps(words[2:],ensure_ascii=False))
    elif words[:2]==['am','start']:
        print('Status: ok')
    elif words[:1]==['pidof']:
        print('12345')
    elif words[:1]==['run-as'] and words[-1:] == ['pwd']:
        print('/'.join(('', 'data','user','0','org.femacs')))
    sys.exit(0)
if args[:1]==['exec-out'] and state.get('capture_fail'):
    print('capture unavailable',file=sys.stderr);sys.exit(1)
if args[:1]==['logcat']: sys.exit(0)
print('unsupported fake SDK command '+repr(args),file=sys.stderr);sys.exit(93)
'''


class Fixture(unittest.TestCase):
    def setUp(self):
        (REPO / 'build').mkdir(exist_ok=True)
        self.temp = tempfile.TemporaryDirectory(prefix='g4-workflow-', dir=REPO / 'build')
        self.root = Path(self.temp.name)
        self.addCleanup(self.temp.cleanup)


class DeviceLifecycle(Fixture):
    @classmethod
    def setUpClass(cls):
        cls.device = load('g4_real_device_workflow', REPO / 'tools/android-device.py')

    def prepare(self, **scenario):
        sdk = self.root / 'sdk'
        apk = self.root / 'test.apk'; so = self.root / 'libemacs.so'
        so.write_bytes(b'owned test library')
        with zipfile.ZipFile(apk, 'w') as archive:
            for name, data in {'AndroidManifest.xml':b'manifest', 'classes.dex':b'dex',
                'resources.arsc':b'resources','lib/arm64-v8a/libemacs.so':so.read_bytes(),
                'assets/directory-tree':b'EMACS','assets/lisp/loadup.el':b'(message "fixture")'}.items():
                archive.writestr(name, data)
        with socket.socket() as listener:
            listener.bind(('127.0.0.1', 0))
            port = listener.getsockname()[1]
        port = port - port % 2
        serial = f'emulator-{port}'
        state = dict(serial=serial, apk=str(apk), so=str(so), avd_name='g4-owned', **scenario)
        config = self.root / 'state.json'; config.write_text(json.dumps(state))
        body = 'STATE_PATH=' + repr(str(config)) + '\n' + SDK_FAKE
        for name in ('platform-tools/adb','emulator/emulator','cmdline-tools/latest/bin/avdmanager'):
            executable(sdk / name, body)
        image = sdk / 'system-images/android-36/default/arm64-v8a'
        image.mkdir(parents=True)
        (image / 'source.properties').write_text('AndroidVersion.ApiLevel=36\nSystemImage.Abi=arm64-v8a\n')
        (image / 'package.xml').write_text('<sdk-repository/>')
        spec = self.device.DeviceSpec('g4-owned', self.root/'run/avd', serial, port,
                  'system-images;android-36;default;arm64-v8a',36,'arm64-v8a')
        product = self.device.Product(apk,so,sha(apk),sha(so),'org.femacs')
        return spec, product, sdk

    def session(self, **scenario):
        spec, product, sdk = self.prepare(**scenario)
        session = self.device.DeviceSession(spec,product,self.root/'run',sdk,
                                            boot_timeout=2,launch_timeout=2)
        self.addCleanup(session.stop)
        return session

    def events(self):
        log = self.root / 'events.jsonl'
        return [json.loads(row) for row in log.read_text().splitlines()] if log.exists() else []

    def assert_owned_clients(self):
        for event in self.events():
            if event['name'] != 'adb': continue
            args = event['args']
            if 'nodaemon' in args or args == ['version']: continue
            self.assertIn('-s', args, event)
            self.assertNotIn('kill-server', args, event)
            self.assertNotIn('root', args, event)
            self.assertNotIn('uninstall', args, event)
            self.assertNotIn('logcat -c', ' '.join(args), event)

    def test_boot_timeout_is_failure_and_owned_emulator_is_reaped(self):
        session = self.session(boot_fail=True)
        session.boot_timeout=.15
        session.start()
        with self.assertRaises(Exception): session.wait_boot()
        session.stop()
        self.assertTrue((self.root/'stop-emulator').is_file())
        self.assertFalse((self.root/'installed').exists())
        self.assert_owned_clients()
        pid = int((self.root/'emulator.pid').read_text())
        with self.assertRaises(ProcessLookupError): os.kill(pid,0)
        if (self.root/'server.pid').exists():
            server_pid=int((self.root/'server.pid').read_text())
            with self.assertRaises(ProcessLookupError):os.kill(server_pid,0)

    def test_install_failure_cannot_launch_or_fall_back_to_old_package(self):
        session = self.session(install_fail=True)
        session.start(); session.wait_boot()
        with self.assertRaises(Exception): session.install()
        session.stop()
        self.assertFalse((self.root/'installed').exists())
        self.assertFalse(any('am start' in ' '.join(row['args']) for row in self.events()))
        self.assert_owned_clients()

    def test_stale_installed_apk_or_library_is_rejected(self):
        session = self.session(stale_product=True)
        session.start(); session.wait_boot()
        with self.assertRaises(Exception): session.install()
        session.stop(); self.assert_owned_clients()

    def test_successful_install_binds_the_actual_apk_and_native_hashes(self):
        session=self.session();session.start();session.wait_boot()
        receipt=session.install()
        self.assertEqual(receipt['status'],'ok')
        self.assertEqual(receipt['device_apk_sha256'],sha(self.root/'test.apk'))
        self.assertEqual(receipt['device_so_sha256'],sha(self.root/'libemacs.so'))
        session.stop();self.assert_owned_clients()

    def test_modern_legacy_native_root_still_requires_the_actual_abi_library_hash(self):
        session=self.session(legacy_native_layout=True);session.start();session.wait_boot()
        receipt=session.install()
        self.assertEqual(session.native_dir,str(self.root/'installed/lib/arm64'))
        self.assertEqual(receipt['device_so_sha256'],sha(self.root/'libemacs.so'))
        self.assertEqual(receipt['device_apk_sha256'],sha(self.root/'test.apk'))
        session.stop();self.assert_owned_clients()

    def test_failed_capture_is_not_a_successful_screenshot(self):
        session=self.session(capture_fail=True);session.start();session.wait_boot()
        with self.assertRaises(Exception):session.capture('current')
        session.stop();self.assert_owned_clients()

    def test_wrong_device_abi_cannot_be_called_the_requested_image(self):
        session = self.session(wrong_abi=True)
        session.start()
        with self.assertRaises(Exception): session.wait_boot()
        session.stop(); self.assert_owned_clients()

    def test_remote_shell_preserves_literal_arguments_and_empty_words(self):
        session = self.session(); session.start(); session.wait_boot()
        values = ['space value', "quote'\"", '中文', '$()', '`touch sentinel`', '']
        result = session.shell(['printf','%s',*values])
        self.assertEqual(json.loads(result), values)
        self.assertFalse((self.root/'sentinel').exists())
        session.stop(); self.assert_owned_clients()

    def test_command_output_is_streamed_and_small_limit_keeps_failure_evidence(self):
        session=self.session()
        session.out.mkdir(parents=True)
        writer=executable(self.root/'bounded-writer','import sys;sys.stdout.write("x"*64)')
        with self.assertRaises(RuntimeError):
            session.command([writer],max_output=16)
        self.assertEqual((session.out/'command-0.stdout').stat().st_size,64)
        self.assertTrue((session.out/'command-0.stderr').exists())
        self.assertEqual(session.events[0]['exit_code'],0)
        session.stop()

    def test_stop_is_idempotent_and_does_not_touch_a_foreign_avd(self):
        foreign = self.root/'foreign-avd'; foreign.mkdir()
        sentinel = foreign/'keep'; sentinel.write_bytes(b'foreign')
        session = self.session(); session.start(); session.wait_boot()
        session.stop(); session.stop()
        self.assertEqual(sentinel.read_bytes(),b'foreign')
        self.assert_owned_clients()

    def test_existing_or_mismatched_avd_is_rejected_without_cleanup(self):
        spec, product, sdk = self.prepare()
        foreign = self.root/'user-avd'; foreign.mkdir()
        sentinel=foreign/'keep'; sentinel.write_bytes(b'foreign')
        wrong = self.device.DeviceSpec(spec.avd_name,foreign,'emulator-1',spec.port,
                                      spec.system_image,36,'arm64-v8a')
        session=None
        try:
            with self.assertRaises(Exception):
                session=self.device.DeviceSession(wrong,product,self.root/'run',sdk,
                                                   boot_timeout=2,launch_timeout=2)
                session.start()
        finally:
            if session is not None: session.stop()
        self.assertEqual(sentinel.read_bytes(),b'foreign')
        self.assertFalse(any(row['name']=='emulator' for row in self.events()))


MANUAL_TOOL_FAKE = r'''
import json,sys
from pathlib import Path
config=Path(CONFIG_PATH);state=json.loads(config.read_text());args=sys.argv[1:]
with (config.parent/'manual-events.jsonl').open('a') as log:
    log.write(json.dumps({'name':Path(sys.argv[0]).name,'args':args,'cwd':str(Path.cwd())})+'\n')
name=Path(sys.argv[0]).name
if '--version' in args:
    print('Femacs 32.0.50' if name=='owned-editor' else 'Texinfo 7.2');sys.exit(0)
if name=='owned-editor':
    assert '-Q' in args and '--batch' in args and 'ox-texinfo' in args
    assert 'org--inhibit-version-check' in ' '.join(args)
    assert 'org-id-track-globally' in ' '.join(args)
    assert Path('org-setup.org').is_file()
    assert Path('../../lisp/org/org.el').is_file()
    assert Path('../../lisp/org/ox-texinfo.el').is_file()
    assert str((Path.cwd()/'../../lisp/org').resolve()) in ' '.join(args)
    assert state['source_misc']+'/../../lisp/org' not in ' '.join(args)
    assert Path.cwd().resolve()!=Path(state['source_misc']).resolve()
    if state.get('fail_export'):sys.exit(4)
    output=Path(args[-1]);assert not output.is_absolute()
    stem=output.stem
    output.write_text('@setfilename '+stem+'.info\n@node Top\n@top '+stem+'\n')
elif name=='makeinfo':
    assert '--no-split' in args
    target=Path(args[args.index('-o')+1]);target.parent.mkdir(parents=True,exist_ok=True)
    if state.get('fail_conversion') and target.name.startswith('alpha'):
        target.write_text('partial invalid result');sys.exit(3)
    assert all(Path(args[i+1]).is_dir() for i,a in enumerate(args[:-1]) if a=='-I')
    includes=[Path(args[i+1]) for i,a in enumerate(args[:-1]) if a=='-I']
    for line in Path(args[-1]).read_text().splitlines():
        if line.startswith('@include '):
            filename=line.split(None,1)[1]
            if not any((directory/filename).is_file() for directory in [Path(args[-1]).parent,*includes]):
                print('missing include '+filename,file=sys.stderr);sys.exit(42)
    target.write_text('<html>Top</html>' if '--html' in args else 'File: '+target.name+', Node: Top\nowned manual\n')
elif name=='install-info':
    option=next((a.split('=',1)[1] for a in args if a.startswith('--dir-file=')),None)
    target=Path(option) if option else Path(args[-1])
    target.parent.mkdir(parents=True,exist_ok=True)
    with target.open('a') as f:f.write('* '+Path(args[0]).name+': (owned).\n')
else:sys.exit(99)
'''


class ManualWorkflows(Fixture):
    def prepare(self, **scenario):
        root=self.root/'source';root.mkdir()
        script=root/'tools/build-manuals.py';script.parent.mkdir()
        shutil.copy2(REPO/'tools/build-manuals.py',script)
        shutil.copy2(REPO/'tools/version_resources.py',root/'tools/version_resources.py')
        rows=[{'source':'doc/misc/alpha.texi','info_name':'alpha','language':'en','org_source':False},
              {'source':'doc/misc/org.org','info_name':'org','language':'en','org_source':True},
              {'source':'doc/misc/modus-themes.org','info_name':'modus-themes','language':'en','org_source':True}]
        inputs={
            'doc/misc/alpha.texi':'@setfilename alpha.info\n@node Top\n@top Alpha\n@include doclicense.texi\n',
            'doc/misc/org.org':'#+setupfile: org-setup.org\n#+title: Org\n',
            'doc/misc/modus-themes.org':'#+title: Modus\n',
            'doc/misc/org-setup.org':'#+macro: version (eval "1.0")\n',
            'doc/misc/doclicense.texi':'Owned fixture license\n',
            'doc/emacs/docstyle.texi':'@c fixture style\n',
            'doc/emacs/emacsver.texi.in':'@set EMACSVER @version@\n',
            'etc/refcards/emacsver.tex.in':'@majorversion@\n',
            'lisp/org/org.el':';; Version: 1.0\n',
            'lisp/org/ox-texinfo.el':';; exporter identity fixture\n',
            'src/emacs.rs':'const PACKAGE_VERSION: &CStr = c"32.0.50";\n',
        }
        for name,contents in inputs.items():
            path=root/name;path.parent.mkdir(parents=True,exist_ok=True);path.write_text(contents)
        support=root/'build-support';support.mkdir()
        manifest={'schema':'femacs-manuals-v1','manuals':rows,'excluded':[],
                  'include_roots':['doc/emacs','doc/misc'],'staged_inputs':sorted(inputs)}
        (support/'manuals.json').write_text(json.dumps(manifest))
        (support/'generator-recipes.json').write_text('{}')
        shutil.copy2(REPO/'tools/bootstrap-resources.py',root/'tools/bootstrap-resources.py')
        (root/'info').mkdir();(root/'info/previous').write_bytes(b'previous published set')
        config=self.root/'manual-fake.json'
        config.write_text(json.dumps(dict(source_misc=str(root/'doc/misc'),**scenario)))
        bin_dir=self.root/'bin'
        body='CONFIG_PATH='+repr(str(config))+'\n'+MANUAL_TOOL_FAKE
        for name in ('makeinfo','install-info','owned-editor'):executable(bin_dir/name,body)
        env=dict(os.environ,PATH=str(bin_dir)+os.pathsep+os.environ.get('PATH',''))
        for key in ('EMACSLOADPATH','EMACSDATA','EMACSDOC','INFOPATH'):env.pop(key,None)
        return root,script,bin_dir/'owned-editor',env,manifest

    def invoke(self, root, script, program, env, *, format='info', out=None):
        out=out or root/'build/manual-test'
        result=subprocess.run([sys.executable,str(script),'--program',str(program),
                               '--format',format,'--out',str(out)],cwd=root,env=env,
                              stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True,timeout=20)
        return result,out

    def test_org_staging_version_inputs_and_complete_info_are_owned(self):
        root,script,program,env,manifest=self.prepare()
        before={name:sha(root/name) for name in manifest['staged_inputs']}
        result,out=self.invoke(root,script,program,env)
        self.assertEqual(result.returncode,0,result.stdout)
        self.assertTrue((out/'info/dir').is_file())
        for name in ('alpha','org','modus-themes'):self.assertTrue((out/'info'/f'{name}.info').is_file())
        self.assertFalse((root/'doc/misc/org.texi').exists())
        self.assertFalse((root/'doc/misc/modus-themes.texi').exists())
        self.assertEqual(before,{name:sha(root/name) for name in before})
        self.assertEqual((root/'info/previous').read_bytes(),b'previous published set')
        receipt=json.loads((out/'receipt.json').read_text())
        self.assertEqual(receipt['schema'],'femacs-manuals-v1')
        self.assertFalse(receipt['errors'])

    def test_export_failure_and_partial_conversion_remain_failures(self):
        for scenario in ('fail_export','fail_conversion'):
            with self.subTest(scenario=scenario):
                # Every fixture is independent, including its old published set.
                nested=ManualWorkflows('test_org_staging_version_inputs_and_complete_info_are_owned')
                nested.setUp()
                try:
                    root,script,program,env,_=nested.prepare(**{scenario:True})
                    result,out=nested.invoke(root,script,program,env)
                    self.assertNotEqual(result.returncode,0,result.stdout)
                    self.assertEqual((root/'info/previous').read_bytes(),b'previous published set')
                    receipt=json.loads((out/'receipt.json').read_text())
                    self.assertTrue(receipt['errors'])
                finally:nested.doCleanups()

    def test_missing_staged_include_cannot_fall_back_to_source_tree(self):
        root,script,program,env,manifest=self.prepare()
        manifest['staged_inputs'].remove('doc/misc/doclicense.texi')
        (root/'build-support/manuals.json').write_text(json.dumps(manifest))
        result,_=self.invoke(root,script,program,env)
        self.assertNotEqual(result.returncode,0,result.stdout)
        self.assertTrue((root/'doc/misc/doclicense.texi').is_file())
        self.assertEqual((root/'info/previous').read_bytes(),b'previous published set')

    def test_duplicate_missing_and_escaping_manifest_inputs_fail(self):
        root,script,program,env,manifest=self.prepare()
        for name in ('duplicate','missing','escaping'):
            bad=copy.deepcopy(manifest)
            if name=='duplicate':bad['manuals'].append(bad['manuals'][0])
            elif name=='missing':bad['staged_inputs'].append('doc/misc/not-there.texi')
            else:bad['staged_inputs'].append('../not-owned.texi')
            (root/'build-support/manuals.json').write_text(json.dumps(bad))
            result,_=self.invoke(root,script,program,env,out=root/f'build/{name}')
            self.assertNotEqual(result.returncode,0,result.stdout)
            self.assertEqual((root/'info/previous').read_bytes(),b'previous published set')

    def test_reused_and_reference_outputs_are_never_modified(self):
        root,script,program,env,_=self.prepare()
        occupied=root/'build/occupied';occupied.mkdir(parents=True)
        sentinel=occupied/'keep';sentinel.write_bytes(b'old evidence')
        reference=root/'emacs';reference.mkdir();foreign=reference/'keep';foreign.write_bytes(b'reference')
        for output in (occupied,reference/'new-output'):
            result,_=self.invoke(root,script,program,env,out=output)
            self.assertNotEqual(result.returncode,0,result.stdout)
        self.assertEqual(sentinel.read_bytes(),b'old evidence')
        self.assertEqual(foreign.read_bytes(),b'reference')
        self.assertFalse((reference/'new-output').exists())

    def test_optional_html_and_missing_makeinfo_do_not_silently_skip(self):
        root,script,program,env,_=self.prepare()
        result,out=self.invoke(root,script,program,env,format='html')
        self.assertEqual(result.returncode,0,result.stdout)
        self.assertTrue((out/'html/alpha.html').is_file())
        (self.root/'bin/makeinfo').unlink()
        empty=self.root/'empty-bin';empty.mkdir()
        result,_=self.invoke(root,script,program,dict(env,PATH=str(empty)),out=root/'build/missing-tool')
        self.assertNotEqual(result.returncode,0,result.stdout)


class RenderRegionRepair(Fixture):
    def setUp(self):
        super().setUp()
        self.device=load('g4_render_region_contract',REPO/'tools/android-device.py')
        self.observation={'frame_width':100,'frame_height':100,'window_edges':[0,0,100,100],
                          'char_height':12,'text':'HELLO','modified':True,'run_token':'current-run',
                          'display_pattern':[{'color':color,'rect':[i*12,0,i*12+12,12]}
                                             for i,color in enumerate(self.device.pattern_colors('current-run'))]}
        self.xml=b'<hierarchy><node package="android" bounds="[0,0][100,20]"/><node package="org.femacs" bounds="[0,20][100,120]"/></hierarchy>'

    def test_status_bar_changes_cannot_pass_editor_render(self):
        from PIL import Image,ImageDraw
        before=self.root/'before.png';after=self.root/'after.png'
        Image.new('RGB',(100,120),'white').save(before)
        image=Image.open(before);ImageDraw.Draw(image).rectangle((0,0,99,19),fill='black');image.save(after)
        with self.assertRaises(Exception):
            self.device.rendering(after,self.observation,ui_xml=self.xml,before=before)

    def test_current_editor_change_uses_global_app_bounds_and_excludes_status(self):
        from PIL import Image,ImageDraw
        before=self.root/'before.png';after=self.root/'after.png'
        Image.new('RGB',(100,120),'white').save(before)
        image=Image.open(before);draw=ImageDraw.Draw(image)
        for cell in self.observation['display_pattern']:
            x,y,r,b=cell['rect'];draw.rectangle((x,y+20,r-1,b+19),fill=cell['color'])
        image.save(after)
        evidence=self.device.rendering(after,self.observation,ui_xml=self.xml,before=before)
        self.assertEqual(evidence['editor_region'],[0,20,100,120])
        self.assertGreater(evidence['changed_pixels'],24)

    def test_nontext_overlay_and_wrong_run_pattern_never_pass(self):
        from PIL import Image,ImageDraw
        before=self.root/'before.png';after=self.root/'after.png'
        Image.new('RGB',(100,120),'white').save(before)
        for wrong in ('overlay','old-run'):
            image=Image.open(before);draw=ImageDraw.Draw(image)
            if wrong=='overlay':draw.rectangle((0,20,99,90),fill='black')
            else:
                for cell,color in zip(self.observation['display_pattern'],self.device.pattern_colors(wrong)):
                    x,y,r,b=cell['rect'];draw.rectangle((x,y+20,r-1,b+19),fill=color)
            image.save(after)
            with self.assertRaises(RuntimeError):self.device.rendering(after,self.observation,ui_xml=self.xml,before=before)

    def test_missing_or_ambiguous_current_app_geometry_cannot_pass(self):
        for xml in (b'<hierarchy/>',b'<hierarchy><node package="org.femacs" bounds="[0,0][99,99]"/></hierarchy>'):
            with self.assertRaises(Exception):
                self.device.content_region(self.observation,xml,(100,120))


class InitialGeometryCoherence(Fixture):
    def setUp(self):
        super().setUp()
        self.device=load('g4_initial_geometry',REPO/'tools/android-device.py')
        self.xml=b'<hierarchy><node package="org.femacs" bounds="[0,20][100,120]"/></hierarchy>'
        self.current={'frame_width':100,'frame_height':100,'window_edges':[0,0,100,100]}
        self.old=dict(self.current,frame_width=576,frame_height=555)
        self.session=mock.Mock(out=self.root)
        self.captures=[]
        def capture(name):
            from PIL import Image
            path=self.root/(name+'.png')
            Image.new('RGB',(100,120),'white').save(path)
            self.captures.append(name)
            return path
        self.session.capture.side_effect=capture
        def ui(name):
            (self.root/(name+'.xml')).write_bytes(self.xml)
            return self.xml
        self.session.current_ui.side_effect=ui

    def test_resize_between_samples_requires_a_second_current_pair(self):
        observe=mock.Mock(side_effect=[self.old,self.current,self.current,self.current])
        actual,image,xml=self.device.coherent_capture(self.session,observe,'empty-editor')
        self.assertEqual(actual,self.current)
        self.assertEqual(image.name,'empty-editor-2.png')
        self.assertEqual(xml,self.xml)
        self.assertEqual(self.captures,['empty-editor','empty-editor-2'])
        evidence=json.loads((self.root/'empty-editor-geometry.json').read_text())
        self.assertEqual([a['passed'] for a in evidence['attempts']],[False,True])
        self.assertTrue((self.root/'empty-editor.png').is_file())

    def test_stale_pair_cannot_override_the_real_client_dimensions(self):
        observe=mock.Mock(side_effect=[self.old,self.old,self.current,self.current])
        actual,image,_=self.device.coherent_capture(self.session,observe,'empty-editor')
        self.assertEqual(actual,self.current)
        self.assertEqual(image.name,'empty-editor-2.png')
        self.assertEqual(observe.call_count,4)

    def test_permanent_mismatch_fails_and_retains_all_three_attempts(self):
        with self.assertRaisesRegex(RuntimeError,'coherent client geometry'):
            self.device.coherent_capture(self.session,lambda:self.old,'empty-editor')
        self.assertEqual(len(self.captures),3)
        evidence=json.loads((self.root/'empty-editor-geometry.json').read_text())
        self.assertEqual([a['passed'] for a in evidence['attempts']],[False]*3)
        self.assertEqual(len(list(self.root.glob('empty-editor*.png'))),3)
        self.assertEqual(len(list(self.root.glob('empty-editor*-ui.xml'))),3)

    def test_identity_failure_stops_before_capture_or_retry(self):
        observe=mock.Mock(side_effect=TimeoutError('observation identity mismatch'))
        with self.assertRaises(TimeoutError):
            self.device.coherent_capture(self.session,observe,'empty-editor')
        self.assertEqual(observe.call_count,1)
        self.session.capture.assert_not_called()
        evidence=json.loads((self.root/'empty-editor-geometry.json').read_text())
        self.assertEqual(len(evidence['attempts']),1)
        self.assertEqual(evidence['attempts'][0]['error_type'],'TimeoutError')

    def test_truncated_pixel_data_cannot_pass_dimensions_only(self):
        from PIL import Image
        path=self.session.capture('damaged')
        data=path.read_bytes();path.write_bytes(data[:len(data)//2])
        with Image.open(path) as pixels:self.assertEqual(pixels.size,(100,120))
        self.session.capture.side_effect=None;self.session.capture.return_value=path
        with self.assertRaises(OSError):
            self.device.coherent_capture(self.session,lambda:self.current,'empty-editor')
        evidence=json.loads((self.root/'empty-editor-geometry.json').read_text())
        self.assertEqual(len(evidence['attempts']),1)
        self.assertFalse(evidence['attempts'][0]['passed'])

    def test_malformed_ui_fails_immediately_and_retains_attempt(self):
        self.session.current_ui.side_effect=lambda _:b'<hierarchy>'
        with self.assertRaises(Exception):
            self.device.coherent_capture(self.session,lambda:self.current,'empty-editor')
        self.assertEqual(len(self.captures),1)
        evidence=json.loads((self.root/'empty-editor-geometry.json').read_text())
        self.assertEqual(evidence['attempts'][0]['error_type'],'ParseError')

    def test_capture_failure_is_preserved_without_retry(self):
        self.session.capture.side_effect=OSError('capture failed')
        with self.assertRaisesRegex(OSError,'capture failed'):
            self.device.coherent_capture(self.session,lambda:self.current,'empty-editor')
        self.session.capture.assert_called_once()
        evidence=json.loads((self.root/'empty-editor-geometry.json').read_text())
        self.assertEqual(evidence['attempts'][0]['error_type'],'OSError')
        self.assertEqual(evidence['attempts'][0]['before'],self.current)

    def test_after_observation_failure_preserves_before_and_capture(self):
        observe=mock.Mock(side_effect=[self.current,TimeoutError('after identity')])
        with self.assertRaisesRegex(TimeoutError,'after identity'):
            self.device.coherent_capture(self.session,observe,'empty-editor')
        self.assertEqual(observe.call_count,2)
        evidence=json.loads((self.root/'empty-editor-geometry.json').read_text())
        self.assertEqual(len(evidence['attempts']),1)
        self.assertEqual(evidence['attempts'][0]['before'],self.current)
        self.assertTrue((self.root/evidence['attempts'][0]['image']).is_file())
        self.assertEqual(evidence['attempts'][0]['error_type'],'TimeoutError')


class KeyMovementEvidence(Fixture):
    def setUp(self):
        super().setUp()
        self.device=load('g4_key_movement',REPO/'tools/android-device.py')
        self.session=mock.Mock(out=self.root)

    def test_each_real_request_retains_its_matching_observation(self):
        observe=mock.Mock(side_effect=[{'point':n,'text':'HELLO'} for n in (1,6,5,6)])
        result=self.device.key_movements(self.session,observe)
        self.assertEqual([r['point'] for r in result],[1,6,5,6])
        evidence=json.loads((self.root/'key-movement-observations.json').read_text())
        self.assertEqual([r['passed'] for r in evidence['attempts']],[True]*4)
        self.assertEqual([r['expected_point'] for r in evidence['attempts']],[1,6,5,6])
        self.assertTrue(all(r['elapsed_seconds']>=0 for r in evidence['attempts']))
        self.assertEqual(self.session.shell.call_args_list[2],mock.call(['input','keyevent','21']))

    def test_later_failure_retains_successes_and_exact_failed_request(self):
        observe=mock.Mock(side_effect=[{'point':1},{'point':6},TimeoutError('point stayed6')])
        with self.assertRaisesRegex(TimeoutError,'point stayed6'):
            self.device.key_movements(self.session,observe)
        evidence=json.loads((self.root/'key-movement-observations.json').read_text())
        self.assertEqual([r['passed'] for r in evidence['attempts']],[True,True,False])
        self.assertEqual(evidence['attempts'][-1]['argv'],['input','keyevent','21'])
        self.assertEqual(evidence['attempts'][-1]['expected_point'],5)
        self.assertEqual(evidence['attempts'][-1]['error_type'],'TimeoutError')
        self.assertEqual(self.session.shell.call_count,3)


class InfoIdentityRepair(Fixture):
    def test_each_named_manual_must_resolve_to_its_exact_delivered_file(self):
        acceptance=load('g4_info_identity',REPO/'tools/g4-acceptance.py')
        info=self.root/'info';info.mkdir()
        manifest=self.root/'manifest.json';manifest.write_text(json.dumps({'manuals':[{'info_name':'alpha'},{'info_name':'beta'}]}))
        for name in ('alpha','beta'):(info/(name+'.info')).write_text('owned')
        class FakeRunner:
            out=self.root
            actual=[]
            def run(this,name,argv,**kwargs):
                (this.out/(name+'.json')).write_text(json.dumps({'schema':'femacs-info-observation-v1','pid':123,'non_top_node':'Basic','manuals':this.actual}))
                return True
            def observation(this,name,passed,detail):this.passed=passed
        runner=FakeRunner()
        runner.actual=[{'name':name,'node':'Top','file':str(info/(name+'.info'))} for name in ('alpha','beta')]
        self.assertTrue(acceptance.info_lookup(runner,'correct',self.root/'editor',info,manifest,{}))
        runner.actual[1]['file']=str(info/'alpha.info')
        self.assertFalse(acceptance.info_lookup(runner,'wrong-file',self.root/'editor',info,manifest,{}))
        runner.actual[1]['file']=str(self.root/'foreign.info')
        self.assertFalse(acceptance.info_lookup(runner,'foreign-file',self.root/'editor',info,manifest,{}))
        # Info's logical stem is intentionally distinct from the physical file.
        # Validate observed behavior instead of prescribing a source expression.
        runner.actual=[{'name':name,'node':'Top','logical_file':str(info/name),
                        'file':str(info/(name+'.info'))} for name in ('alpha','beta')]
        self.assertTrue(acceptance.info_lookup(runner,'logical-stems',self.root/'editor',info,manifest,{}))
        runner.actual[0]['node']='wrong'
        self.assertFalse(acceptance.info_lookup(runner,'wrong-node',self.root/'editor',info,manifest,{}))
        runner.actual[0]['node']='Top'
        runner.actual.pop()
        self.assertFalse(acceptance.info_lookup(runner,'missing-manual',self.root/'editor',info,manifest,{}))


class FileControlMeasurement(unittest.TestCase):
    def test_target_integer_data_parses_both_compilers_and_requires_the_whole_family(self):
        gate=load('g4_file_control_contract',REPO/'tools/check-android-constants.py')
        c='target datalayout = "e-m:e"\n@femacs_flag_O_CREAT = constant i64 64, align 8\n@femacs_flag_AT_FDCWD = constant i64 -100, align 8\n'
        rust='target datalayout = "e-m:e"\n@femacs_flag_O_CREAT = constant [8 x i8] c"@\\00\\00\\00\\00\\00\\00\\00", align 8\n@femacs_flag_AT_FDCWD = constant [8 x i8] c"\\9C\\FF\\FF\\FF\\FF\\FF\\FF\\FF", align 8\n'
        names=('O_CREAT','AT_FDCWD')
        self.assertEqual(gate.llvm_flag_values(c,names),{'O_CREAT':64,'AT_FDCWD':-100})
        self.assertEqual(gate.llvm_flag_values(rust,names),gate.llvm_flag_values(c,names))
        for bad in (c.replace('e-m:e','E-m:e'),c.replace('@femacs_flag_O_CREAT','@missing'),c+c.splitlines()[1]+'\n',rust.replace('[8 x i8]','[4 x i8]')):
            with self.assertRaises(ValueError):gate.llvm_flag_values(bad,names)
        self.assertIn('O_CREAT',gate.FILE_FLAGS)
        self.assertIn('F_DUPFD_CLOEXEC',gate.FILE_FLAGS)
        self.assertIn('AT_FDCWD',gate.FILE_FLAGS)


class AndroidRuntimeInitialization(unittest.TestCase):
    def test_all_android_runtime_initializers_precede_loadup_in_original_order(self):
        source=(REPO/'src/emacs.rs').read_text()
        block=source.split('{ init_androidmenu (); }',1)[1].split('if !initialized',1)[0]
        self.assertIn('#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]',block)
        calls=re.findall(r'\b(init_androidfont|init_androidselect|init_sfntfont|init_sfntfont_android)\s*\(\s*\)\s*;',block)
        self.assertEqual(calls,['init_androidfont','init_androidselect','init_sfntfont','init_sfntfont_android'])
        for name in calls:
            self.assertEqual(len(re.findall(r'\bfn '+name+r'\s*\(',source)),1)


class FailureDiagnosticsRepair(Fixture):
    def test_pending_checkjni_abort_is_fatal_even_while_the_process_exists(self):
        device=load('g4_native_fault_contract',REPO/'tools/android-device.py')
        for marker in (b'Fatal signal 6',b'FATAL EXCEPTION',b'Abort message',
                       b'dlopen failed',b'cannot locate symbol',
                       b'JNI DETECTED ERROR IN APPLICATION: jmethodID was NULL'):
            self.assertEqual(device.native_fault_count(marker),1)
        self.assertEqual(device.native_fault_count(b'Initializing runtime; native thread running'),0)

    def test_diagnostics_keep_other_evidence_after_one_read_failure_and_never_accept(self):
        device=load('g4_failure_diagnostic_contract',REPO/'tools/android-device.py')
        class Receiver:
            pid=None
            calls=[]
            def adb_command(this,argv,**kwargs):
                this.calls.append(argv)
                self.assertLessEqual(kwargs['max_output'],4*1024*1024)
                if argv[-1]=='dumpsys window windows':raise RuntimeError('owned window unavailable')
                return b'owned diagnostic'
            def capture(this,name):
                path=self.root/(name+'.png');path.write_bytes(b'owned image fixture');return path
        receiver=Receiver();result=device.failure_diagnostics(receiver,self.root)
        self.assertFalse(result['acceptance'])
        self.assertEqual(result['kind'],'diagnostic')
        self.assertEqual(len(receiver.calls),4)
        self.assertEqual(len(result['entries']),5)
        self.assertEqual([r['status'] for r in result['entries']].count('unavailable'),1)
        self.assertTrue((self.root/'failure-main.log').is_file())
        self.assertTrue((self.root/'failure-screen.png').is_file())
        receiver.pid=731;receiver.calls=[]
        result=device.failure_diagnostics(receiver,self.root)
        self.assertFalse(result['acceptance'])
        self.assertEqual(receiver.calls[0],['logcat','-d','--pid=731','-t','512','-v','threadtime'])
        self.assertEqual(len(receiver.calls),5)
        self.assertTrue((self.root/'failure-app.log').is_file())


class ArchiveModeRepair(Fixture):
    def test_wrap_mode_changes_only_central_attributes_and_preserves_member_offsets(self):
        archive=load('g4_archive_mode_contract',REPO/'tools/android-archive.py')
        path=self.root/'aligned.apk'
        with zipfile.ZipFile(path,'w') as file:
            file.writestr('classes.dex',b'owned dex')
            file.writestr('lib/arm64-v8a/wrap.sh',b'#!/system/bin/sh\nexec "$@"\n')
        before=path.read_bytes()
        with zipfile.ZipFile(path) as file:
            offsets={entry.filename:entry.header_offset for entry in file.infolist()}
            content={entry.filename:file.read(entry) for entry in file.infolist()}
        archive.patch_wrap_mode(path)
        after=path.read_bytes()
        self.assertEqual(len(before),len(after))
        self.assertLessEqual(sum(a!=b for a,b in zip(before,after)),4)
        with zipfile.ZipFile(path) as file:
            self.assertEqual(offsets,{entry.filename:entry.header_offset for entry in file.infolist()})
            self.assertEqual(content,{entry.filename:file.read(entry) for entry in file.infolist()})
            self.assertEqual(file.getinfo('lib/arm64-v8a/wrap.sh').external_attr>>16,0o100755)


class CleanupRecovery(unittest.TestCase):
    def test_all_g4_withdrawals_have_real_git_recovery_and_final_identity(self):
        inventory=json.loads((REPO/'maintenance/tooling-inventory.json').read_text())
        manifests=inventory['cleanup_manifests']
        self.assertGreaterEqual(len(manifests),4)
        self.assertEqual(len(manifests),len(set(manifests)))
        ledger=(REPO/'maintenance/cleanup/LOG.md').read_text()
        with tempfile.TemporaryDirectory(prefix='g4-recovery-',dir=REPO/'build') as temp:
            recovery=Path(temp)
            restored=0
            for name in manifests:
                path=Path(name)
                self.assertFalse(path.is_absolute());self.assertNotIn('..',path.parts)
                manifest=json.loads((REPO/path).read_text())
                self.assertIn(manifest['id'],ledger)
                self.assertRegex(manifest['source_ref'],r'^[0-9a-f]{40}$')
                self.assertTrue(manifest['source_files'])
                for item in manifest['source_files']:
                    relative=Path(item['path'])
                    self.assertFalse(relative.is_absolute());self.assertNotIn('..',relative.parts)
                    old=subprocess.run(['git','show',manifest['source_ref']+':'+relative.as_posix()],
                                       cwd=REPO,check=True,stdout=subprocess.PIPE).stdout
                    self.assertEqual(hashlib.sha256(old).hexdigest(),item['before_sha256'])
                    destination=recovery/manifest['id']/relative
                    destination.parent.mkdir(parents=True,exist_ok=True);destination.write_bytes(old)
                    self.assertEqual(sha(destination),item['before_sha256'])
                    current=REPO/relative
                    if item.get('after_sha256') is None:self.assertFalse(current.exists(),str(relative))
                    else:self.assertEqual(sha(current),item['after_sha256'],str(relative))
                    restored+=1
            self.assertGreater(restored,3)

    def test_support_scope_is_current_and_other_platform_sources_are_retained(self):
        text=(REPO/'docs/tooling-modernization.md').read_text()
        for phrase in ('API21','API36','retained/deferred','latest LTS','68'):
            self.assertIn(phrase,text)
        for path in ('src/w32fns.rs','src/haikufns.rs','tools/zig-linux-linker.sh',
                     'lisp','leim','doc','admin'):
            self.assertTrue((REPO/path).exists(),path)


class AndroidLayoutOwnership(unittest.TestCase):
    """Real output/CLI boundaries; only NDK measurement is replaced here."""

    def setUp(self):
        cache = REPO / '.femacs-cache'
        cache.mkdir(exist_ok=True)
        self.temporary = tempfile.TemporaryDirectory(prefix='g4-layout-owner-', dir=cache)
        self.addCleanup(self.temporary.cleanup)
        self.path = Path(self.temporary.name)
        self.root = self.path / 'repo'
        self.root.mkdir()
        self.gate = load('layout_owner_' + self._testMethodName,
                         REPO / 'tools/check-android-layout.py')
        self.gate.REPO = self.root

    def cli_output(self, location):
        destination = self.root / location
        bins = self.path / 'fake-ndk'
        with contextlib.ExitStack() as stack:
            stack.enter_context(mock.patch.object(sys, 'argv', [
                'layout', '--quiet', '--ndk-bin', str(bins), '--out', str(destination)]))
            stack.enter_context(mock.patch.object(self.gate, 'rows',
                return_value=[('Record', '', '', 'sizeof(void*)', '8', 'fixture')]))
            measured = stack.enter_context(mock.patch.object(self.gate, 'measure', return_value=8))
            stack.enter_context(mock.patch.object(self.gate, 'rust_assertions', return_value=[]))
            stack.enter_context(contextlib.redirect_stdout(io.StringIO()))
            self.assertEqual(self.gate.main(), 0)
        measured.assert_called_once_with(bins, 'sizeof(void*)', [])
        self.assertEqual(self.gate.RUN_ROOT, destination.resolve())
        self.assertTrue(destination.is_dir())

    def test_actual_cli_accepts_fresh_build_child(self):
        self.cli_output('build/measurements/current')

    def test_actual_cli_accepts_fresh_cache_child(self):
        self.cli_output('.femacs-cache/measurements/current')

    def test_existing_output_is_rejected_without_changes(self):
        destination = self.root / '.femacs-cache/existing'
        destination.mkdir(parents=True)
        marker = destination / 'keep'
        marker.write_bytes(b'original')
        with self.assertRaises(SystemExit):
            self.gate.new_run_directory(destination)
        self.assertEqual(marker.read_bytes(), b'original')

    def test_foreign_path_is_rejected_before_creation(self):
        destination = self.path / 'foreign-output'
        with self.assertRaises(SystemExit):
            self.gate.new_run_directory(destination)
        self.assertFalse(destination.exists())

    def test_parent_symlink_escape_is_rejected(self):
        foreign = self.path / 'foreign'
        foreign.mkdir()
        (self.root / 'build').mkdir()
        (self.root / 'build/link').symlink_to(foreign, target_is_directory=True)
        with self.assertRaises(SystemExit):
            self.gate.new_run_directory(self.root / 'build/link/output')
        self.assertEqual(list(foreign.iterdir()), [])

    def test_reference_link_is_rejected_before_writes(self):
        reference = self.root / 'emacs'
        reference.mkdir()
        (self.root / '.femacs-cache').mkdir()
        (self.root / '.femacs-cache/link').symlink_to(reference, target_is_directory=True)
        with self.assertRaises(SystemExit):
            self.gate.new_run_directory(self.root / '.femacs-cache/link/output')
        self.assertEqual(list(reference.iterdir()), [])

    def test_default_root_link_cannot_write_to_reference(self):
        reference = self.root / 'emacs'
        reference.mkdir()
        (self.root / 'build').symlink_to(reference, target_is_directory=True)
        with self.assertRaises(SystemExit):
            self.gate.new_run_directory(None)
        self.assertEqual(list(reference.iterdir()), [])

    def test_dangling_leaf_link_is_not_a_fresh_output(self):
        cache = self.root / '.femacs-cache'
        cache.mkdir()
        link = cache / 'link'
        link.symlink_to(cache / 'new', target_is_directory=True)
        with self.assertRaises(SystemExit):
            self.gate.new_run_directory(link)
        self.assertTrue(link.is_symlink())
        self.assertFalse((cache / 'new').exists())


class TimedResult(unittest.TextTestResult):
    def startTest(self,test):self.case_started=time.monotonic();super().startTest(test)
    def stopTest(self,test):
        self.stream.writeln(f'CASE {test.id()} seconds={time.monotonic()-self.case_started:.6f}')
        super().stopTest(test)


if __name__=='__main__':
    unittest.main(testRunner=unittest.TextTestRunner(verbosity=2,resultclass=TimedResult))
