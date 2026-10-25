#!/usr/bin/env python3
"""Execute extracted real Kotlin methods against both source snapshots in D.

The Android API objects here are deterministic JVM fakes. This proves selected
method/branch equivalence for SDK21+; it does not certify any Android device.
The complete Kotlin/dex build and current API36 device tests are separate.
Run under the external build resource controller, after all G4 C is written.
"""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import time

REPO=Path(__file__).resolve().parent.parent
BASELINE='01bfbb7f3e1d8e6793a2dd71b81ac8ba43b29f99'
APIS=(21,22,23,24,26,30,36)
METHODS=(('EmacsService.kt','getLibraryDirectory'),
         ('EmacsService.kt','nameKeysym'),('EmacsWindow.kt','eventModifiers'))


def extract_method(text: str, name: str) -> str:
    match=re.search(r'\bfun\s+'+re.escape(name)+r'\s*\([^)]*\)\s*:\s*[^\n{]+\{',text)
    if match is None:raise ValueError('missing actual Kotlin method '+name)
    start=match.start();i=match.end()-1;depth=0;state='code';block_depth=0
    while i<len(text):
        char=text[i];pair=text[i:i+2]
        if state=='line':
            if char=='\n':state='code'
        elif state=='block':
            if pair=='/*':block_depth+=1;i+=1
            elif pair=='*/':
                block_depth-=1;i+=1
                if block_depth==0:state='code'
        elif state in ('string','char'):
            if char=='\\':i+=1
            elif char==('"' if state=='string' else "'"):state='code'
        elif pair=='//':state='line';i+=1
        elif pair=='/*':state='block';block_depth=1;i+=1
        elif char=='"':state='string'
        elif char=="'":state='char'
        elif char=='{':depth+=1
        elif char=='}':
            depth-=1
            if depth==0:return text[start:i+1]
        i+=1
    raise ValueError('unclosed method '+name)


BUILD_STUB='''package android.os
object Build {
  object VERSION { @JvmField var SDK_INT:Int=21 }
  object VERSION_CODES {
    const val GINGERBREAD=9; const val HONEYCOMB=11
    const val HONEYCOMB_MR1=12; const val HONEYCOMB_MR2=13
  }
}
'''
KEY_STUB='''package android.view
class KeyEvent(private val state:Int) {
  fun getMetaState():Int=state
  companion object {
    const val META_ALT_LEFT_ON=16;const val META_ALT_RIGHT_ON=32
    const val META_CTRL_LEFT_ON=8192;const val META_CTRL_RIGHT_ON=16384
    const val META_ALT_MASK=50;const val META_CTRL_MASK=28672
    @JvmStatic fun normalizeMetaState(value:Int):Int=value xor 0x5a5a
    @JvmStatic fun keyCodeToString(value:Int):String="key-code-"+value
  }
}
'''
CONTEXT_STUB='''package android.content
class ApplicationInfo {
  @JvmField val nativeLibraryDir:String="modern-native-directory"
  @JvmField val dataDir:String="legacy-data-directory"
}
class Context { fun getApplicationInfo():ApplicationInfo=ApplicationInfo() }
'''
MAIN='''package floor
import android.os.Build
import android.view.KeyEvent
import android.content.Context
fun main() {
  for (api in intArrayOf(21,22,23,24,26,30,36)) {
    Build.VERSION.SDK_INT=api
    val started=System.nanoTime()
    check(Before.getLibraryDirectory(Context())==After.getLibraryDirectory(Context()))
    check(After.getLibraryDirectory(Context())=="modern-native-directory")
    for (key in intArrayOf(0,1,29,47,1000))
      check(Before.nameKeysym(key)==After.nameKeysym(key))
    for (mask in intArrayOf(0,1,16,32,50,8192,16384,28672,65535)) {
      check(Before.eventModifiers(KeyEvent(mask))==After.eventModifiers(KeyEvent(mask)))
      check(After.eventModifiers(KeyEvent(mask))==KeyEvent.normalizeMetaState(mask))
    }
    println("CASE sdk="+api+" passed=true seconds="+(System.nanoTime()-started)/1e9)
  }
}
'''


def write(path: Path, text: str) -> Path:
    path.write_text(text,encoding='utf-8');return path


def main(argv=None) -> int:
    ap=argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--toolchain-report',type=Path,required=True)
    ap.add_argument('--out',type=Path,required=True)
    args=ap.parse_args(argv)
    tools=json.loads(args.toolchain_report.read_text())['toolchain']
    if tools['jdk']['major']!=25:ap.error('requires the recorded current LTS25 toolchain')
    jdk=Path(tools['jdk']['home']).resolve();kotlinc=Path(tools['kotlinc']).resolve()
    if not (jdk/'bin/java').is_file() or not kotlinc.is_file():ap.error('missing recorded toolchain')
    out=args.out.resolve()
    if not out.is_relative_to((REPO/'build').resolve()) or out==(REPO/'build').resolve():
        ap.error('requires fresh owned build output')
    out.mkdir(parents=True,exist_ok=False)
    sources=[];identities={}
    for group,before in (('Before',True),('After',False)):
        methods=[]
        for filename,name in METHODS:
            relative='platform/android/kotlin/'+filename
            if before:
                source=subprocess.run(['git','show',BASELINE+':'+relative],cwd=REPO,
                                      check=True,stdout=subprocess.PIPE).stdout.decode()
            else:source=(REPO/relative).read_text()
            method=extract_method(source,name);methods.append(method)
            identities[group+'.'+name]=hashlib.sha256(method.encode()).hexdigest()
        sources.append(write(out/(group+'.kt'),
            'package floor\nimport android.os.Build\nimport android.view.KeyEvent\n'
            'import android.content.Context\nobject '+group+' {\n'+'\n'.join(methods)+'\n}\n'))
    for name,text in [('Build.kt',BUILD_STUB),('KeyEvent.kt',KEY_STUB),
                      ('Context.kt',CONTEXT_STUB),('Main.kt',MAIN)]:
        sources.append(write(out/name,text))
    env=dict(os.environ,JAVA_HOME=str(jdk))
    jar=out/'floor.jar';started=time.monotonic()
    with (out/'compile.log').open('xb') as log:
        result=subprocess.run([str(kotlinc),*[str(p) for p in sources],'-jvm-target','17',
                               '-include-runtime','-d',str(jar)],env=env,cwd=REPO,
                              stdout=log,stderr=subprocess.STDOUT)
    compile_seconds=time.monotonic()-started
    if result.returncode:
        receipt={'passed':False,'stage':'compile','exit_code':result.returncode,
                 'compile_seconds':compile_seconds,'method_sha256':identities}
    else:
        started=time.monotonic()
        with (out/'run.log').open('xb') as log:
            result=subprocess.run([str(jdk/'bin/java'),'-jar',str(jar)],env=env,cwd=out,
                                  stdout=log,stderr=subprocess.STDOUT,timeout=30)
        elapsed=time.monotonic()-started
        rows=[]
        for line in (out/'run.log').read_text(errors='replace').splitlines():
            match=re.fullmatch(r'CASE sdk=(\d+) passed=true seconds=([0-9.eE+-]+)',line)
            if match:rows.append({'sdk':int(match[1]),'passed':True,'seconds':float(match[2])})
        receipt={'passed':result.returncode==0 and [r['sdk'] for r in rows]==list(APIS),
                 'stage':'matrix','exit_code':result.returncode,'compile_seconds':compile_seconds,
                 'run_seconds':elapsed,'cases':rows,'method_sha256':identities}
    receipt.update(schema='femacs-api-floor-v1',source_baseline=BASELINE,
                   scope='extracted actual method equivalence with JVM API fakes',
                   android_device_certification=False)
    (out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
    print(json.dumps(receipt))
    return 0 if receipt['passed'] else 1


if __name__=='__main__':raise SystemExit(main())
