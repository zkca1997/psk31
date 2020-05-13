extern crate timer;
extern crate chrono;

use chrono::Duration;

pub struct Sampler {
    ts: Duration,
    guard: timer::Guard,
}

impl Sampler {
    pub fn new(fs: f64) -> Self {
        let ts = Duration::nanoseconds((1e9 / fs) as i64);
        let timer = timer::Timer::new();
        let guard = timer.schedule_repeating(ts, move || println!("foo") );
        Self { ts, guard }
    }

    pub fn kill(self) {
        drop(self.guard);
    }
}
