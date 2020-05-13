mod costa;
mod filter;
mod driver;
mod sampler;

use std::{thread, time};

const Fs: f64 = 4e3;        // sampling frequency
const Fc: f64 = 4e2;        // carrier frequency
const Rb: f64 = 31.25;      // baud rate

fn main() {
    let sampler = sampler::Sampler::new(1.0);
    thread::sleep(time::Duration::new(5, 0));
}
