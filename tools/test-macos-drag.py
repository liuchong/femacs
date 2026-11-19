#!/usr/bin/env python3
"""Real external Finder drag-and-drop acceptance for the macOS Cocoa bundle.

Opens an owned temporary directory in a new Finder icon-view window, launches a
fresh bundled editor instance with tools/fixtures/macos-drag.el, drags the owned
file icon from Finder into the target native view using HID mouse events, and
verifies both configurable ns-drag-motion-function callbacks (draggingUpdated)
and actual file drop opening (performDragOperation).
"""
from __future__ import annotations

import argparse
import ctypes
import ctypes.util
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parent.parent
SCHEMA = 'femacs-macos-external-drag-v1'
TARGET_TITLE = 'Femacs Drag Target'


def new_output(path: Path) -> Path:
    resolved = (path if Path(path).is_absolute() else (ROOT / path)).resolve()
    reference = (ROOT / 'emacs').resolve()
    if resolved == reference or reference in resolved.parents:
        raise ValueError('output must not be inside the read-only reference tree')
    if os.path.lexists(path) or resolved.exists():
        raise FileExistsError(f'output already exists: {path}')
    resolved.mkdir(parents=True, exist_ok=False)
    return resolved


def sha256_file(path: Path) -> dict:
    target = Path(path).resolve()
    if not target.is_file():
        raise FileNotFoundError(f'missing product file: {path}')
    digest = hashlib.sha256()
    with target.open('rb') as handle:
        for block in iter(lambda: handle.read(1024 * 1024), b''):
            digest.update(block)
    return {'path': str(target), 'bytes': target.stat().st_size, 'sha256': digest.hexdigest()}


def resolve_app_bundle(target: Path) -> tuple[Path, Path, Path]:
    candidate = Path(target).resolve()
    app = candidate if candidate.suffix == '.app' else None
    if app is None:
        for parent in candidate.parents:
            if parent.suffix == '.app':
                app = parent
                break
    if app is None or not app.is_dir():
        raise ValueError(f'expected a macOS .app bundle or executable inside one: {target}')
    executable = app / 'Contents/MacOS/Femacs'
    dump = app / 'Contents/Resources/libexec/Femacs.pdmp'
    if not executable.is_file() or not os.access(executable, os.X_OK):
        raise FileNotFoundError(f'missing bundle executable: {executable}')
    if not dump.is_file():
        raise FileNotFoundError(f'missing bundle dump: {dump}')
    return app, executable, dump


def read_callbacks(path: Path) -> list[dict]:
    try:
        raw = Path(path).read_bytes()
    except FileNotFoundError:
        return []
    if len(raw) > 1_048_576:
        raise ValueError('native callback trace exceeds 1 MiB')
    rows = []
    for line in raw.splitlines(keepends=True):
        if not line.endswith(b'\n'):
            break
        item = json.loads(line)
        if not isinstance(item, dict):
            raise ValueError('callback trace line must be a JSON object')
        rows.append(item)
    return rows


def validate_motion_events(events: object, expected_title: str = TARGET_TITLE) -> bool:
    if not isinstance(events, list) or not events:
        return False
    for row in events:
        if not isinstance(row, dict):
            return False
        if row.get('frame-live') is not True or row.get('name') != expected_title:
            return False
        for axis in ('x', 'y'):
            coord = row.get(axis)
            if type(coord) is not int:
                return False
    return True


def validate_drag_receipt(ready_state: object, motion_events: object,
                          callbacks: list[dict], expected_content: str,
                          expected_pid: int) -> tuple[bool, str | None]:
    if not isinstance(ready_state, dict):
        return False, 'missing ready snapshot object'
    if ready_state.get('pid') != expected_pid or ready_state.get('native') != 'ns':
        return False, 'launched editor PID or window-system mismatch'
    if ready_state.get('frame') != TARGET_TITLE:
        return False, 'unexpected target frame title in ready snapshot'
    if ready_state.get('opened') != expected_content:
        return False, 'dropped file content was not opened with exact UTF-8 bytes'
    if not validate_motion_events(motion_events, TARGET_TITLE):
        return False, 'ns-drag-motion-function did not record live frame and integer coordinates'
    names = {row.get('event') for row in callbacks if isinstance(row, dict)}
    if not {'draggingUpdated', 'performDragOperation'} <= names:
        return False, 'native callback trace lacks draggingUpdated or performDragOperation'
    return True, None


def pid_executable_path(pid: int) -> str:
    libproc = ctypes.CDLL(ctypes.util.find_library('proc'))
    buf = ctypes.create_string_buffer(4096)
    size = libproc.proc_pidpath(int(pid), buf, len(buf))
    return buf.value.decode('utf-8', 'replace') if size > 0 else ''


def check_desktop_prerequisites() -> dict:
    app_services = ctypes.CDLL('/System/Library/Frameworks/ApplicationServices.framework/ApplicationServices')
    app_services.AXIsProcessTrusted.restype = ctypes.c_bool
    trusted = bool(app_services.AXIsProcessTrusted())
    if not trusted:
        raise RuntimeError('Accessibility permission is required for external Finder drag acceptance')
    cg = ctypes.CDLL('/System/Library/Frameworks/CoreGraphics.framework/CoreGraphics')
    cf = ctypes.CDLL('/System/Library/Frameworks/CoreFoundation.framework/CoreFoundation')
    cg.CGSessionCopyCurrentDictionary.restype = ctypes.c_void_p
    cf.CFRelease.argtypes = [ctypes.c_void_p]
    session_dict = cg.CGSessionCopyCurrentDictionary()
    if not session_dict:
        raise RuntimeError('no active graphical CGSession dictionary')
    cf.CFRelease(session_dict)
    return {'ax_trusted': trusted, 'cg_session_present': True}


def post_hid_drag(start_xy: tuple[float, float], end_xy: tuple[float, float], steps: int = 25) -> None:
    cg = ctypes.CDLL('/System/Library/Frameworks/CoreGraphics.framework/CoreGraphics')
    cf = ctypes.CDLL('/System/Library/Frameworks/CoreFoundation.framework/CoreFoundation')

    class CGPoint(ctypes.Structure):
        _fields_ = [('x', ctypes.c_double), ('y', ctypes.c_double)]

    cg.CGEventSourceCreate.argtypes = [ctypes.c_int32]
    cg.CGEventSourceCreate.restype = ctypes.c_void_p
    cg.CGEventCreateMouseEvent.argtypes = [ctypes.c_void_p, ctypes.c_uint32, CGPoint, ctypes.c_uint32]
    cg.CGEventCreateMouseEvent.restype = ctypes.c_void_p
    cg.CGEventPost.argtypes = [ctypes.c_uint32, ctypes.c_void_p]
    cf.CFRelease.argtypes = [ctypes.c_void_p]

    source = cg.CGEventSourceCreate(1)
    if not source:
        raise RuntimeError('CGEventSourceCreate failed')
    try:
        def emit(event_type: int, x: float, y: float) -> None:
            event = cg.CGEventCreateMouseEvent(source, event_type, CGPoint(x, y), 0)
            if not event:
                raise RuntimeError(f'CGEventCreateMouseEvent({event_type}) failed')
            cg.CGEventPost(0, event)
            cf.CFRelease(event)

        sx, sy = start_xy
        tx, ty = end_xy
        emit(5, sx, sy)
        time.sleep(0.10)
        emit(1, sx, sy)
        time.sleep(0.18)
        for index in range(1, steps + 1):
            frac = index / float(steps)
            emit(6, sx + (tx - sx) * frac, sy + (ty - sy) * frac)
            time.sleep(0.02)
        time.sleep(0.25)
        emit(6, tx + 4.0, ty + 4.0)
        time.sleep(0.15)
        emit(2, tx + 4.0, ty + 4.0)
    finally:
        cf.CFRelease(source)


def run_drag_acceptance(app_target: Path, out_dir: Path, timeout: float = 30.0) -> dict:
    work = new_output(out_dir)
    started = time.monotonic()
    app, executable, dump = resolve_app_bundle(app_target)
    before = {'executable': sha256_file(executable), 'dump': sha256_file(dump)}
    result: dict = {
        'schema': SCHEMA,
        'passed': False,
        'app': str(app),
        'products_before': before,
    }
    finder_window_id: str | None = None
    launched_pid: int | None = None
    try:
        result['desktop'] = check_desktop_prerequisites()
        source_dir = work / 'finder-source'
        source_dir.mkdir(parents=True, exist_ok=False)
        sample_name = 'femacs-drag-sample.txt'
        sample_file = source_dir / sample_name
        expected_content = 'External Finder drag payload 你好 UTF-8 2026\n'
        sample_file.write_text(expected_content, encoding='utf-8')

        open_script = (
            'tell application "Finder"\n'
            '  activate\n'
            f'  set fwin to make new Finder window to (POSIX file "{source_dir}")\n'
            '  set current view of fwin to icon view\n'
            '  set bounds of fwin to {40, 80, 580, 480}\n'
            f'  select (POSIX file "{sample_file}" as alias)\n'
            '  return id of fwin\n'
            'end tell\n'
        )
        finder_window_id = subprocess.check_output(
            ['osascript', '-e', open_script], text=True, timeout=15).strip()
        time.sleep(0.4)

        helper = work / 'ime-launch'
        built = subprocess.run(
            ['swiftc', '-O', str(ROOT / 'tools/fixtures/ime-launch.swift'), '-o', str(helper)],
            capture_output=True, text=True, timeout=30)
        if built.returncode != 0:
            raise RuntimeError('failed to compile LaunchServices helper: ' + built.stderr[:300])

        ready_path = work / 'ready.json'
        motion_path = work / 'motion.json'
        trace_path = work / 'callbacks.jsonl'
        launch_spec = work / 'launch.json'
        env = dict(
            os.environ,
            HOME=str(work / 'home'),
            FEMACS_DRAG_FILE=str(sample_file),
            FEMACS_DRAG_READY=str(ready_path),
            FEMACS_DRAG_REPORT=str(motion_path),
            FEMACS_INPUT_TRACE=str(trace_path),
        )
        (work / 'home').mkdir(parents=True, exist_ok=True)
        launch_argv = ['-Q', '-name', TARGET_TITLE, '-l', str(ROOT / 'tools/fixtures/macos-drag.el')]
        launch_spec.write_text(json.dumps({'arguments': launch_argv, 'environment': env}), encoding='utf-8')
        launched = subprocess.run(
            [str(helper), str(app), str(launch_spec)], capture_output=True, text=True, timeout=25)
        if launched.returncode != 0:
            raise RuntimeError('LaunchServices open failed: ' + launched.stderr[:300])
        launched_pid = int(launched.stdout.strip().splitlines()[-1])
        actual_image = pid_executable_path(launched_pid)
        if not actual_image or os.path.realpath(actual_image) != os.path.realpath(str(executable)):
            raise RuntimeError(f'launched PID {launched_pid} image mismatch: {actual_image!r}')
        result['launch'] = {
            'pid': launched_pid,
            'image': os.path.realpath(actual_image),
            'finder_window_id': finder_window_id,
        }

        deadline = time.monotonic() + timeout
        ready_state = None
        while time.monotonic() < deadline:
            if ready_path.is_file():
                try:
                    ready_state = json.loads(ready_path.read_text(encoding='utf-8'))
                    if ready_state.get('native') == 'ns' and ready_state.get('pid') == launched_pid:
                        break
                except ValueError:
                    pass
            time.sleep(0.08)
        else:
            raise RuntimeError('timed out waiting for target native drag frame readiness')

        jxa = (
            'ObjC.import("stdlib");\n'
            'const se = Application("System Events");\n'
            'const finder = se.processes.byName("Finder");\n'
            f'const targetProc = se.processes.whose({{unixId: {launched_pid}}})[0];\n'
            'const tpos = targetProc.windows[0].position();\n'
            'const tsz = targetProc.windows[0].size();\n'
            'finder.frontmost = true;\n'
            'delay(0.3);\n'
            'function findIcon(el, depth) {\n'
            '  if (depth > 9) return null;\n'
            '  try {\n'
            '    const role = el.role();\n'
            '    let name = ""; try { name = String(el.name()); } catch (e) {}\n'
            '    let title = ""; try { title = String(el.title()); } catch (e) {}\n'
            f'    if (role === "AXImage" && (name === "{sample_name}" || title === "{sample_name}")) {{\n'
            '      const pos = el.position(); const sz = el.size();\n'
            '      if (sz[0] >= 16 && sz[1] >= 16) return {role, pos, sz};\n'
            '    }\n'
            '    const kids = el.uiElements();\n'
            '    for (let i = 0; i < kids.length; i++) {\n'
            '      const hit = findIcon(kids[i], depth + 1);\n'
            '      if (hit) return hit;\n'
            '    }\n'
            '  } catch (e) {}\n'
            '  return null;\n'
            '}\n'
            'JSON.stringify({source: findIcon(finder.windows[0], 0), target: {pos: tpos, size: tsz}});\n'
        )
        geometry = json.loads(subprocess.check_output(
            ['osascript', '-l', 'JavaScript', '-e', jxa], text=True, timeout=15).strip())
        icon = geometry.get('source')
        target_geom = geometry.get('target')
        if not isinstance(icon, dict) or not isinstance(target_geom, dict):
            raise RuntimeError(f'could not resolve Finder icon or target window geometry: {geometry!r}')
        sx = float(icon['pos'][0]) + float(icon['sz'][0]) * 0.5
        sy = float(icon['pos'][1]) + float(icon['sz'][1]) * 0.35
        tx = float(target_geom['pos'][0]) + float(target_geom['size'][0]) * 0.5
        ty = float(target_geom['pos'][1]) + float(target_geom['size'][1]) * 0.5
        result['drag_coordinates'] = {'source': [sx, sy], 'target': [tx, ty], 'geometry': geometry}

        post_hid_drag((sx, sy), (tx, ty))

        final_state = None
        drop_deadline = time.monotonic() + min(12.0, timeout)
        while time.monotonic() < drop_deadline:
            if ready_path.is_file():
                try:
                    final_state = json.loads(ready_path.read_text(encoding='utf-8'))
                    if final_state.get('opened') == expected_content:
                        break
                except ValueError:
                    pass
            time.sleep(0.08)

        motion_events = (json.loads(motion_path.read_text(encoding='utf-8'))
                         if motion_path.is_file() else [])
        callbacks = read_callbacks(trace_path)
        ok, reason = validate_drag_receipt(
            final_state, motion_events, callbacks, expected_content, launched_pid)
        if not ok:
            raise RuntimeError(reason or 'drag verification failed')
        result['motion_event_count'] = len(motion_events)
        result['motion_sample'] = motion_events[:4]
        result['callback_count'] = len(callbacks)
        result['callback_events'] = sorted({r.get('event') for r in callbacks if isinstance(r.get('event'), str)})
        result['opened_bytes'] = len(expected_content.encode('utf-8'))
        result['ready_state'] = final_state
        result['passed'] = True
    except Exception as exc:
        result['error'] = f'{type(exc).__name__}: {exc}'
        result['passed'] = False
    finally:
        if helper_path := (work / 'ime-launch'):
            if helper_path.is_file():
                helper_path.unlink()
        if launched_pid is not None:
            try:
                os.kill(launched_pid, signal.SIGTERM)
                for _ in range(40):
                    try:
                        os.kill(launched_pid, 0)
                    except ProcessLookupError:
                        break
                    time.sleep(0.05)
                else:
                    os.kill(launched_pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
        if finder_window_id is not None:
            close_script = (
                'tell application "Finder" to try\n'
                f'  close Finder window id {finder_window_id}\n'
                'end try\n'
            )
            subprocess.run(['osascript', '-e', close_script],
                           stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, check=False)
        after = {'executable': sha256_file(executable), 'dump': sha256_file(dump)}
        result['products_after'] = after
        result['products_unchanged'] = (before == after)
        if not result['products_unchanged']:
            result['passed'] = False
        result['seconds'] = time.monotonic() - started
        (work / 'results.json').write_text(
            json.dumps(result, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    return result


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--app', type=Path, default=ROOT / 'dist/macos/Femacs.app',
                        help='macOS .app bundle or executable inside one')
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--timeout', type=float, default=30.0)
    args = parser.parse_args(argv)
    summary = run_drag_acceptance(args.app, args.out, args.timeout)
    print(json.dumps({'passed': summary['passed'], 'seconds': summary['seconds'],
                      'error': summary.get('error')}, ensure_ascii=False))
    return 0 if summary['passed'] else 1


if __name__ == '__main__':
    raise SystemExit(main())
