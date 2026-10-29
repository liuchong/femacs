"""Protect P432's actual Swift caller after native background/GC acceptance.

Compile the production function against disjoint frame/face sentinels. Also
restore the old wrong argument in a private mutant and require its rejection.
This checks receiver and branch behavior, not Cocoa layout or real rendering.
Run under controlled-run build protection with a fresh owned output.
"""
import argparse
import importlib.util
import json
import os
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parent.parent

SUPPORT = r'''
import Foundation
import Darwin
typealias Lisp_Object = Int64
typealias EmacsCGFloat = Double
struct NSColor {
  var pixel: UInt
  var alphaComponent: Double
  func withAlphaComponent(_ alpha: Double) -> NSColor {
    NSColor(pixel: (pixel & 0xffffff) | (UInt(alpha * 255) << 24), alphaComponent: alpha)
  }
}
final class TestWindow { var backgroundColor: NSColor?; var isOpaque = false }
final class TestView { var window: TestWindow? = TestWindow() }
var testFrame = UnsafeMutableRawPointer.allocate(byteCount: 64, alignment: 8)
var testFace = UnsafeMutableRawPointer.allocate(byteCount: 64, alignment: 8)
var testView: TestView?
var hasFace = true
var visible = true
var alpha = 1.0
var blockDepth = 0
var storedPixel: UInt = 0
var faceWrites = 0
var wrongReceiver = false
var invalidations = 0
var clears = 0
func block_input() { blockDepth += 1 }
func unblock_input() { blockDepth -= 1 }
func FRAME_NS_VIEW(_ f: UnsafeMutableRawPointer?) -> TestView? { testView }
func ns_lisp_to_color(_ a: Lisp_Object, _ color: inout NSColor?) -> Int32 {
  color = NSColor(pixel: 0xff123456, alphaComponent: alpha); return 0
}
func Qbackground_color() -> Lisp_Object { 0 }
func store_frame_param(_ f: UnsafeMutableRawPointer?, _ a: Lisp_Object, _ b: Lisp_Object) {}
func error(_ message: String) -> Never { fatalError(message) }
func ns_output_set_background_color(_ f: UnsafeMutableRawPointer?, _ c: NSColor?) {}
func unsignedLong(_ c: NSColor) -> UInt { c.pixel }
func frame_set_background_pixel(_ f: UnsafeMutableRawPointer?, _ v: UInt) { storedPixel = v }
func FRAME_DEFAULT_FACE(_ f: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? { hasFace ? testFace : nil }
func NS_FACE_BACKGROUND(_ f: UnsafeMutableRawPointer?) -> UInt { f!.load(as: UInt.self) }
func NSColor_colorWithUnsignedLong(_ v: UInt) -> NSColor? { NSColor(pixel: v, alphaComponent: 1) }
func face_set_background(_ f: UnsafeMutableRawPointer?, _ v: UInt) {
  faceWrites += 1; wrongReceiver = f != testFace; f!.storeBytes(of: v, as: UInt.self)
}
func update_face_from_frame_parameter(_ f: UnsafeMutableRawPointer?, _ a: Lisp_Object, _ b: Lisp_Object) { invalidations += 1 }
func FRAME_VISIBLE_P(_ f: UnsafeMutableRawPointer?) -> Bool { visible }
func SET_FRAME_GARBAGED(_ f: UnsafeMutableRawPointer?) {}
func ns_clear_frame(_ f: UnsafeMutableRawPointer?) { clears += 1 }
'''

CASES = r'''
var allPassed = true
for (name, viewPresent, facePresent, opacity, expectedPixel) in [
  ("opaque-face", true, true, 1.0, UInt(0xff334455)),
  ("transparent-face", true, true, 0.4, UInt(0x66334455)),
  ("missing-face", true, false, 1.0, UInt(0xff334455)),
  ("missing-view", false, true, 1.0, UInt(0xff334455))
] {
  let started = Date().timeIntervalSinceReferenceDate
  testFrame.initializeMemory(as: UInt8.self, repeating: 0xa5, count: 64)
  testFace.initializeMemory(as: UInt8.self, repeating: 0x5a, count: 64)
  testFace.storeBytes(of: UInt(0xff334455), as: UInt.self)
  testView = viewPresent ? TestView() : nil; hasFace = facePresent; alpha = opacity
  blockDepth = 0; storedPixel = 0; faceWrites = 0; wrongReceiver = false; invalidations = 0; clears = 0
  ns_set_background_color(testFrame, 0, 0)
  let shouldWrite = viewPresent && facePresent
  let frameUnchanged = UnsafeRawBufferPointer(start: testFrame, count: 64).allSatisfy { $0 == 0xa5 }
  let faceTailUnchanged = UnsafeRawBufferPointer(start: testFace.advanced(by: 8), count: 56).allSatisfy { $0 == 0x5a }
  let passed = frameUnchanged && faceTailUnchanged && !wrongReceiver && blockDepth == 0
    && storedPixel == 0xff123456 && testFace.load(as: UInt.self) == expectedPixel
    && faceWrites == (shouldWrite ? 1 : 0) && invalidations == (shouldWrite ? 1 : 0)
    && clears == (viewPresent ? 1 : 0)
    && (!viewPresent || testView!.window!.isOpaque == (opacity == 1))
  allPassed = allPassed && passed
  let row: [String: Any] = ["case": name, "passed": passed, "frame_unchanged": frameUnchanged,
    "wrong_receiver": wrongReceiver, "seconds": Date().timeIntervalSinceReferenceDate - started]
  print(String(data: try JSONSerialization.data(withJSONObject: row), encoding: .utf8)!)
}
testFrame.deallocate(); testFace.deallocate()
exit(allPassed ? 0 : 1)
'''


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, required=True)
    out = parser.parse_args().out.absolute()
    if (not out.is_relative_to(ROOT / 'build') or out == ROOT / 'build'
            or any(p.is_symlink() for p in [out, *out.parents])):
        raise ValueError('fresh nonsymlink build child required')
    out.mkdir(parents=True, exist_ok=False)
    spec = importlib.util.spec_from_file_location('background_receiver_execution', ROOT / 'tools/runtime-execution.py')
    api = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = api
    spec.loader.exec_module(api)
    source = ROOT / 'platform/macos/nsfns.swift'
    before = api.digest(source)
    text = source.read_text()
    match = re.search(r'func ns_set_background_color\(', text)
    if not match:
        raise ValueError('production background setter missing')
    opening = text.index('{', match.start())
    end, depth = opening + 1, 1
    while depth:
        if text[end] == '{':
            depth += 1
        if text[end] == '}':
            depth -= 1
        end += 1
    function = text[match.start():end]
    correct = 'face_set_background(face, unsignedLong(col!.withAlphaComponent(alpha)))'
    if function.count(correct) != 1:
        raise ValueError('production receiver contract changed; inspect actual source')
    mutant = function.replace(correct, correct.replace('(face,', '(f,'))
    report = dict(source_sha256=before, passed=False, runs=[])
    for label, body in [('production', function), ('old-receiver-mutant', mutant)]:
        work = out / label
        work.mkdir()
        (work / 'probe.swift').write_text(SUPPORT + '\n' + body + '\n' + CASES)
        compile_result = api.run_process(['swiftc', '-O', work / 'probe.swift', '-o', work / 'probe'],
                                         work / 'compile', dict(os.environ), 40, cwd=ROOT)
        if compile_result['exit_code'] != 0:
            report['runs'].append(dict(id=label, compile=compile_result, passed=False))
            api.write_json(out / 'results.json', report)
            return 1
        result = api.run_process([work / 'probe'], work / 'execute', dict(os.environ), 5, cwd=ROOT)
        rows = [json.loads(line) for line in (work / 'execute/stdout.txt').read_text().splitlines()]
        failed = {row['case'] for row in rows if row['passed'] is not True}
        expected_failures = {'opaque-face', 'transparent-face'} if label == 'old-receiver-mutant' else set()
        passed = (result['complete'] and not result['remaining_group_terminated'] and len(rows) == 4
                  and failed == expected_failures and result['exit_code'] == (1 if expected_failures else 0))
        report['runs'].append(dict(id=label, compile=compile_result, process=result, cases=rows, passed=passed))
        api.write_json(out / 'results.json', report)
    report['source_unchanged'] = api.digest(source) == before
    report['passed'] = report['source_unchanged'] and all(row['passed'] for row in report['runs'])
    api.write_json(out / 'results.json', report)
    print(json.dumps(dict(passed=report['passed'], production_cases=4, rejected_mutant_cases=2)))
    return 0 if report['passed'] else 1


if __name__ == '__main__':
    raise SystemExit(main())
