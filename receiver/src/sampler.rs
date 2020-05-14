use rppal::uart::{Parity, Uart};

pub struct Sampler {
    uart: Uart,
}

impl Sampler {
    pub fn start() -> Self {
        let ts = Duration::nanoseconds((1e9 / fs) as i64);
        let timer = timer::Timer::new();
        let guard = timer.schedule_repeating(ts, move || println!("foo") );
        Self { ts, guard }
    }

    pub fn kill(self) {
        drop(self.guard);
    }
}
