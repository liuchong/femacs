#!/usr/bin/env python3
"""Protect actual numeric recorder boundaries after real phase integration.

Run under the build controller. The source numeric module is extracted unchanged;
timestamps and calls are explicit test inputs, not a simulated VM or real clock.
"""
from pathlib import Path
import argparse
import hashlib
import json
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


TESTS = r'''
#[cfg(test)]
mod protection {
    use super::numeric::*;
    // Multi-megabyte fixed storage must not be constructed on a test stack.
    static mut RECORDER: Recorder = Recorder::new();
    struct Timer(&'static str, std::time::Instant);
    impl Drop for Timer {
        fn drop(&mut self) {
            println!("CASE_TIME\t{}\t{}", self.0, self.1.elapsed().as_nanos());
        }
    }
    fn begin(name: &'static str, mode: u8) -> (&'static mut Recorder, Timer) {
        let r = unsafe { &mut *(&raw mut RECORDER) };
        r.reset_process(mode);
        r.enabled = true;
        (r, Timer(name, std::time::Instant::now()))
    }
    #[test]
    fn prepare_before_classification_and_sibling_roots_preserve_cycle() {
        let (r, _time) = begin("prepare_before_classification_and_sibling_roots_preserve_cycle", 1);
        for young in [false, true] {
            assert!(r.reset_window());
            let token = r.reserve_cycle(); assert!(r.enter_cycle(token));
            r.phase_begin(token, 0, Some(1)); r.phase_begin(token, 3, Some(2));
            r.phase_begin(token, 22, Some(3)); r.phase_end(token, 22, Some(4));
            r.classification(token, young, 7, Some(5));
            if young {
                r.phase_begin(token, 23, Some(6)); r.phase_end(token, 23, Some(7));
                r.phase_begin(token, 24, Some(8)); r.phase_end(token, 24, Some(9));
            }
            r.phase_begin(token, 4, Some(10)); r.phase_end(token, 4, Some(11));
            r.phase_end(token, 3, Some(12)); r.phase_end(token, 0, Some(13));
            r.end_cycle(token);
            let phases: Vec<u32> = r.records[..r.count].iter().map(|x| x.phase).collect();
            assert_eq!(phases, if young { vec![0,3,22,21,23,24,4] } else { vec![0,3,22,21,4] });
            for record in &r.records[..r.count] {
                assert_eq!(record.cycle, token); assert_eq!(record.parent_cycle, 0);
                let expected = if record.phase == 21 { COMPLETE | (if young {256} else {512}) | (7<<16) } else { COMPLETE };
                assert_eq!(record.flags, expected); assert!(record.end_ns >= record.start_ns);
            }
            assert_eq!(r.current_cycle(),0); assert_eq!(r.depth,0); assert_eq!(r.totals[4],0);
        }
    }
    #[test]
    fn abort_keeps_open_root_intervals_incomplete_and_recorder_reusable() {
        let (r, _time) = begin("abort_keeps_open_root_intervals_incomplete_and_recorder_reusable", 1);
        let token=r.reserve_cycle(); assert!(r.enter_cycle(token));
        r.phase_begin(token,0,Some(1)); r.phase_begin(token,3,Some(2)); r.phase_begin(token,23,Some(3));
        r.abort_cycle(token);
        assert_eq!(r.count,3); assert_eq!(r.depth,0); assert_eq!(r.current_cycle(),0);
        assert_eq!(r.totals[4],1); assert_eq!(r.totals[5],1);
        assert!(r.records[..3].iter().all(|x| x.flags & INCOMPLETE != 0 && x.flags & COMPLETE == 0));
        let next=r.reserve_cycle(); assert!(next>token && r.enter_cycle(next));
        r.phase_begin(next,22,Some(4)); r.phase_end(next,22,Some(5)); r.end_cycle(next);
        assert_eq!(r.records[3].flags,COMPLETE); assert_eq!(r.depth,0);
        assert!(r.records[..3].iter().all(|x| x.flags & COMPLETE == 0));
    }
    #[test]
    fn invalid_clock_never_becomes_complete() {
        let (r, _time) = begin("invalid_clock_never_becomes_complete", 1);
        let token=r.reserve_cycle(); assert!(r.enter_cycle(token));
        r.phase_begin(token,22,Some(10)); r.phase_end(token,22,Some(9));
        r.phase_begin(token,23,None); r.phase_end(token,23,Some(20)); r.end_cycle(token);
        assert_eq!(r.count,2); assert_eq!(r.depth,0);
        assert!(r.records[..2].iter().all(|x| x.flags & CLOCK != 0 && x.flags & COMPLETE == 0));
        assert!(r.totals[4]>0 && r.totals[6]>0);
    }
    #[test]
    fn fixed_capacity_preserves_records_and_disabled_modes_emit_nothing() {
        let (r, _time) = begin("fixed_capacity_preserves_records_and_disabled_modes_emit_nothing", 1);
        let token=r.reserve_cycle(); assert!(r.enter_cycle(token));
        for i in 0..CAPACITY+2 {
            r.phase_begin(token,24,Some(i as u64)); r.phase_end(token,24,Some(i as u64+1));
        }
        r.end_cycle(token);
        assert_eq!(r.count,CAPACITY); assert_eq!(r.totals[3],2); assert_eq!(r.depth,0);
        assert_eq!(r.records[0].start_ns,0); assert_eq!(r.records[CAPACITY-1].start_ns,(CAPACITY-1) as u64);
        assert!(r.records.iter().all(|x| x.flags==COMPLETE));
        for mode in [0,2] {
            r.reset_process(mode); r.enabled=true;
            let token=r.reserve_cycle(); assert_eq!(token,0); assert!(!r.enter_cycle(token));
            for phase in [22,23,24] { r.phase_begin(token,phase,Some(1)); r.phase_end(token,phase,Some(2)); }
            assert_eq!(r.count,0); assert_eq!(r.depth,0); assert_eq!(r.totals[3],0); assert_eq!(r.totals[4],0);
        }
    }
}
'''


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args()
    out = args.out.resolve()
    assert out.is_relative_to(ROOT/'build') and not out.exists()
    source = ROOT/'src/runtime_gc_observation.rs'
    text = source.read_text()
    start = text.index('    pub(super) mod numeric {')
    opening = text.index('{', start)
    depth, end = 1, opening+1
    while depth:
        depth += (text[end]=='{')-(text[end]=='}')
        end += 1
    module = text[start:end].replace('pub(super) mod numeric', 'mod numeric', 1)
    out.mkdir(parents=True)
    (out/'recorder.rs').write_text(module+'\n'+TESTS)
    started = time.monotonic()
    report = dict(passed=False, execution_complete=False, source_sha256=digest(source),
        driver_sha256=digest(Path(__file__)), generated_sha256=digest(out/'recorder.rs'),
        boundary='Unmodified numeric recorder body; injected call sequence and synthetic timestamps. Not VM, clock, owner-thread, cfg-off binary or GC-root acceptance.',
        scale='Four serial tests, one fixed4098-record loop, fixed process-reset arrays; static multi-megabyte storage.',
        budget=dict(whole_seconds=100, compile_seconds=60, execute_seconds=20))
    def save():
        report['seconds'] = time.monotonic()-started
        (out/'results.json').write_text(json.dumps(report,indent=2)+'\n')
    save()
    try:
        for phase, argv, limit in (
            ('compile',['rustc','--edition=2024','--test','-C','opt-level=1','-C','overflow-checks=on','recorder.rs','-o','recorder-test'],60),
            ('execute',[str(out/'recorder-test'),'--test-threads=1','--nocapture'],20)):
            begin=time.monotonic()
            with (out/(phase+'.stdout')).open('w') as stdout, (out/(phase+'.stderr')).open('w') as stderr:
                process=subprocess.run(argv,cwd=out,stdout=stdout,stderr=stderr,timeout=limit)
            report[phase]=dict(exit_code=process.returncode,seconds=time.monotonic()-begin)
            save(); assert process.returncode==0, phase
        assert 'test result: ok. 4 passed; 0 failed;' in (out/'execute.stdout').read_text()
        report.update(execution_complete=True,source_unchanged=report['source_sha256']==digest(source),
                      driver_unchanged=report['driver_sha256']==digest(Path(__file__)))
        report['passed']=report['source_unchanged'] and report['driver_unchanged']; save()
    except BaseException as error:
        report.update(error_type=type(error).__name__,error=str(error)[:1000]); save(); raise
    print(json.dumps(report))


if __name__=='__main__':
    main()
