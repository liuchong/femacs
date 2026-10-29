#!/usr/bin/env python3
"""Measure target C layouts independently of the Rust editor's layout asserts.

This optional differential diagnostic reads an explicit source/configuration
cache; production generation and builds never invoke it or require that cache.
Run this compiler-owning driver under one external build controller.
"""
from __future__ import annotations
import argparse
import importlib.util
import json
import os
from pathlib import Path
import shlex
import sys

from module_api import render_module_api

ROOT = Path(__file__).resolve().parent.parent
spec = importlib.util.spec_from_file_location('module_thread_execution', ROOT / 'tools/runtime-execution.py')
execution = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = execution
spec.loader.exec_module(execution)

MAC_SOURCE = r'''
#include <config.h>
#undef THREADS_ENABLED
#define THREADS_ENABLED 1
#undef HAVE_PTHREAD
#define HAVE_PTHREAD 1
#undef HAVE_MODULES
#define HAVE_MODULES 1
#include <stddef.h>
#include <stdio.h>
#include "lisp.h"
#include "thread.h"
#include "emacs-module.h"
_Static_assert(sizeof(pthread_mutex_t) == 64, "Darwin mutex size");
_Static_assert(_Alignof(pthread_mutex_t) == 8, "Darwin mutex alignment");
_Static_assert(sizeof(pthread_cond_t) == 48, "Darwin condition size");
_Static_assert(_Alignof(pthread_cond_t) == 8, "Darwin condition alignment");
_Static_assert(sizeof(struct thread_state) == 520, "thread state size");
_Static_assert(offsetof(struct thread_state, event_object) == 72, "event field");
_Static_assert(offsetof(struct thread_state, m_specpdl) == 120, "specpdl field");
_Static_assert(offsetof(struct thread_state, m_getcjmp) == 216, "jump field");
_Static_assert(offsetof(struct thread_state, thread_id) == 408, "thread identity");
_Static_assert(offsetof(struct thread_state, not_holding_lock) == 480, "lock state");
_Static_assert(offsetof(struct thread_state, bc) == 496, "bytecode state");
_Static_assert(sizeof(lisp_mutex_t) == 64, "Lisp mutex state");
_Static_assert(sizeof(struct Lisp_Mutex) == 80, "Lisp mutex");
_Static_assert(sizeof(struct Lisp_CondVar) == 72, "Lisp condition");
_Static_assert(sizeof(struct emacs_env_31) == 320, "stable module API prefix");
_Static_assert(sizeof(emacs_env) == 328, "current module API");
_Static_assert(offsetof(emacs_env, canvas_data) == 320, "append-only API slot");
int main(void) {
  printf("{\"thread_state\":%zu,\"not_holding_lock\":%zu,\"bc\":%zu,"
         "\"lisp_mutex\":%zu,\"Lisp_Mutex\":%zu,\"Lisp_CondVar\":%zu,"
         "\"stable_env\":%zu,\"current_env\":%zu}\n",
         sizeof(struct thread_state), offsetof(struct thread_state, not_holding_lock),
         offsetof(struct thread_state, bc), sizeof(lisp_mutex_t),
         sizeof(struct Lisp_Mutex), sizeof(struct Lisp_CondVar),
         sizeof(struct emacs_env_31), sizeof(emacs_env));
  return 0;
}
'''

JNI_SOURCE = r'''
#include <stddef.h>
#include <jni.h>
_Static_assert(sizeof(struct JNIInvokeInterface) == 64, "invocation table size");
_Static_assert(offsetof(struct JNIInvokeInterface, DestroyJavaVM) == 24, "destroy");
_Static_assert(offsetof(struct JNIInvokeInterface, AttachCurrentThread) == 32, "attach");
_Static_assert(offsetof(struct JNIInvokeInterface, DetachCurrentThread) == 40, "detach");
_Static_assert(offsetof(struct JNIInvokeInterface, GetEnv) == 48, "environment");
_Static_assert(offsetof(struct JNIInvokeInterface, AttachCurrentThreadAsDaemon) == 56, "daemon");
_Static_assert(sizeof(JavaVM) == 8 && sizeof(JNIEnv) == 8, "JNI handle types");
_Static_assert(JNI_OK == 0, "success code");
'''

LOADER_SOURCE = r'''
#include <stddef.h>
#include <stdio.h>
#include <dlfcn.h>
_Static_assert(sizeof(Dl_info) == 32, "loader record");
_Static_assert(offsetof(Dl_info, dli_fname) == 0, "image name");
_Static_assert(offsetof(Dl_info, dli_fbase) == 8, "image base");
_Static_assert(offsetof(Dl_info, dli_sname) == 16, "symbol name");
_Static_assert(offsetof(Dl_info, dli_saddr) == 24, "symbol address");
int main(void) {
  Dl_info info = {0};
  int found = dladdr((const void *) dlopen, &info);
  int names = found && info.dli_fname && *info.dli_fname && info.dli_sname && *info.dli_sname;
  printf("{\"record_bytes\":%zu,\"found\":%s,\"names\":%s}\n", sizeof info,
         found ? "true" : "false", names ? "true" : "false");
  return names ? 0 : 1;
}
'''


def input_identity(path):
    return dict(path=str(path), sha256=execution.digest(path), bytes=path.stat().st_size)


def succeeded(process):
    return process['complete'] and process['exit_code'] == 0 and not process['remaining_group_terminated']


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', required=True, type=Path)
    parser.add_argument('--cc', required=True, type=Path)
    parser.add_argument('--sdk-root', required=True, type=Path)
    parser.add_argument('--targets', choices=('both', 'macos', 'android', 'loader'), default='both')
    parser.add_argument('--c-source', required=True, type=Path, help='Explicit read-only source src directory')
    parser.add_argument('--c-config', required=True, type=Path, help='Matching generated config.h directory')
    parser.add_argument('--ndk', required=True, type=Path, help='Existing NDK root; no installation')
    args = parser.parse_args()
    out = args.out.absolute()
    if not out.is_relative_to(ROOT / 'build') or out.resolve() != out or out.exists():
        parser.error('a fresh owned build output is required')
    cc = args.cc.resolve(strict=True)
    source = args.c_source.resolve(strict=True)
    config = args.c_config.resolve(strict=True)
    ndk = args.ndk.resolve(strict=True)
    sdk_root = args.sdk_root.resolve(strict=True)
    if not sdk_root.is_dir(): parser.error("existing actual macOS SDK required")
    for p in (source / 'thread.h', source / 'systhread.h', config / 'config.h'):
        if not p.is_file():
            parser.error('missing explicit ABI source/config input')
    out.mkdir(parents=True)
    header = render_module_api(ROOT)
    inputs = [cc, source / 'thread.h', source / 'systhread.h', source / 'lisp.h',
              config / 'config.h', header, ROOT / 'build-support/module-api/sources.json',
              ROOT / 'src/include/thread.rs', ROOT / 'src/include/systhread.rs',
              ROOT / 'src/include/jni.rs', ROOT / 'src/include/emacs-module.in.rs']
    inputs.append(ROOT / 'src/dynlib.rs')
    before = [input_identity(p) for p in inputs]
    execution.write_json(out / 'inputs.json', dict(schema='femacs-module-thread-abi-inputs-v1', files=before,
        driver_sha256=execution.digest(Path(__file__)), c_thread_gate='THREADS_ENABLED/HAVE_PTHREAD/HAVE_MODULES explicitly enabled after config.h',
        reference_cache_is_optional_diagnostic_only=True))
    env = {k:v for k,v in os.environ.items() if not k.startswith('FEMACS_') and k not in execution.FORBIDDEN_ENV}
    cases = []
    if args.targets in ('both', 'macos'):
        mac = out / 'darwin-thread-module.c'
        mac.write_text(MAC_SOURCE)
        command = [cc, '-isysroot', sdk_root, '-arch', 'arm64', '-std=gnu2x', '-O0', '-MD', '-MF', out / 'darwin-dependencies.d',
                   '-I', header.parent, '-I', config, '-I', config.parent / 'lib',
                   '-I', source, '-I', source.parent / 'lib', mac, '-o', out / 'darwin-abi']
        compile_result = execution.run_process(command, out / 'darwin-compile', env, 120, cwd=ROOT)
        cases.append(dict(id='darwin-C-layout-compile', passed=succeeded(compile_result),
                          process=compile_result))
        if cases[-1]['passed']:
            result = execution.run_process([out / 'darwin-abi'], out / 'darwin-run', env, 30, cwd=ROOT)
            observed = None
            try:
                observed = json.loads((out / 'darwin-run/stdout.txt').read_text())
            except (OSError, ValueError):
                pass
            expected = dict(thread_state=520, not_holding_lock=480, bc=496, lisp_mutex=64,
                            Lisp_Mutex=80, Lisp_CondVar=72, stable_env=320, current_env=328)
            cases.append(dict(id='darwin-C-layout-observation', passed=succeeded(result)
                              and observed == expected, observed=observed, expected=expected, process=result))
        else:
            cases.append(dict(id='darwin-C-layout-observation', passed=False, status='not-run', reason='C layout compilation failed'))
        dependency_record = out / 'darwin-dependencies.d'
        dependencies = []
        if dependency_record.is_file():
            text = dependency_record.read_text().replace('\\\n', ' ')
            for item in shlex.split(text.split(':', 1)[1]):
                path = Path(item)
                if not path.is_absolute(): path = ROOT / path
                dependencies.append(input_identity(path.resolve(strict=True)))
        execution.write_json(out / 'actual-C-inputs.json', dict(files=dependencies,
            scope='actual dependency paths including system headers, discovered after compilation; primary source/config identities are also checked before and after'))
    if args.targets in ('both', 'android'):
        prebuilt = ndk / 'toolchains/llvm/prebuilt'
        hosts = [p for p in prebuilt.iterdir() if (p / 'bin/aarch64-linux-android24-clang').is_file()]
        if len(hosts) != 1:
            raise ValueError('need exactly one actual installed NDK host toolchain')
        host = hosts[0]
        jni = host / 'sysroot/usr/include/jni.h'
        jni_before = input_identity(jni)
        native = out / 'jni-invocation.c'
        native.write_text(JNI_SOURCE)
        result = execution.run_process([host / 'bin/aarch64-linux-android24-clang', '-std=c11', '-fsyntax-only', native],
                                       out / 'android-JNI-compile', env, 120, cwd=ROOT)
        cases.append(dict(id='android-target-JNI-invocation-layout', passed=succeeded(result)
                          and input_identity(jni) == jni_before, process=result, header=jni_before,
                          limitation='target type checking/static asserts only; no Android runtime execution'))
    if args.targets in ('both', 'macos', 'loader'):
        source_file = out / 'darwin-loader.c'
        source_file.write_text(LOADER_SOURCE)
        result = execution.run_process([cc, '-isysroot', sdk_root, '-arch', 'arm64',
            '-std=gnu2x', source_file, '-o', out / 'darwin-loader'],
            out / 'darwin-loader-compile', env, 120, cwd=ROOT)
        cases.append(dict(id='darwin-loader-layout-compile', passed=succeeded(result), process=result))
        if succeeded(result):
            result = execution.run_process([out / 'darwin-loader'], out / 'darwin-loader-run', env, 30, cwd=ROOT)
            try: observed = json.loads((out / 'darwin-loader-run/stdout.txt').read_text())
            except (OSError, ValueError): observed = None
            cases.append(dict(id='darwin-loader-symbol-observation', passed=succeeded(result)
                and observed == dict(record_bytes=32, found=True, names=True), observed=observed, process=result))
        else:
            cases.append(dict(id='darwin-loader-symbol-observation', passed=False,
                status='not-run', reason='loader compilation failed'))
    unchanged = before == [input_identity(p) for p in inputs]
    report = dict(schema='femacs-module-thread-abi-v1', cases=cases, selected_targets=args.targets, original_inputs_unchanged=unchanged,
                  passed=unchanged and all(case['passed'] for case in cases),
                  limitation='C target ABI only; Rust layout asserts still require the actual new editor build; no module/thread behavior or full compatibility acceptance')
    execution.write_json(out / 'results.json', report)
    print(json.dumps(dict(passed=report['passed'], cases=len(cases), out=str(out))))
    return 0 if report['passed'] else 1


if __name__ == '__main__':
    raise SystemExit(main())
