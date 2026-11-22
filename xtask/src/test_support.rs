//! Test-only per-case elapsed time; included only by the final test build.
#![forbid(unsafe_code)]
pub(crate) struct CaseTimer {
    name: &'static str,
    started: std::time::Instant,
}
impl CaseTimer {
    pub(crate) fn new(name: &'static str) -> Self {
        Self {
            name,
            started: std::time::Instant::now(),
        }
    }
}
impl Drop for CaseTimer {
    fn drop(&mut self) {
        println!(
            "CASE {} seconds={:.6}",
            self.name,
            self.started.elapsed().as_secs_f64()
        );
    }
}
