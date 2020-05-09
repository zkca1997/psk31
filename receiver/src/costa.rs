use crate::filter;
use std::f64;

const PI: f64 = f64::consts::PI;

pub struct Costa {
    w_lo: f32,
    theta: f32,
    loop_filt: filter::IIRFilter,
    i_filter:  filter::IIRFilter,
    q_filter:  filter::IIRFilter,
    matched:   filter::FIRFilter,
}

impl Costa {
    pub fn new(fc: f64, fs: f64, rb: f64, g: f64) -> Self {
        let n: i32 = (fs / rb).floor() as i32;
        let w_lo: f32 = (2.0 * PI * fc / fs) as f32;
        let theta = 0.0;
        let i_filter  = filter::IIRFilter::butterworth(9, 2.0*rb, fs);
        let q_filter  = filter::IIRFilter::butterworth(9, 2.0*rb, fs);
        let matched   = filter::FIRFilter::pulse_shaper(n);
        let loop_filt = Costa::loop_filter(fs, rb/g);

        Costa { w_lo, theta, loop_filt, i_filter, q_filter, matched }
    }

    fn loop_filter(fs: f64, w_d: f64) -> filter::IIRFilter {
        let ts: f64   = 1.0 / fs;
        let damp: f64 = (2_f64).sqrt() / 2.0;
        let w_n: f64  = w_d * 8.0 * damp / (4.0*damp*damp + 1.0);
        let den: f64  = 4.0 + 4.0 * damp * w_n * ts + (w_n*ts)*(w_n*ts);
        let c1: f64   = 8.0 * damp * w_n * ts / den;
        let c2: f64   = 4.0 * (w_n*ts)*(w_n*ts) / den;
        filter::IIRFilter::new(&[(c1+c2) as f32, (-1.0*c1) as f32], &[1.0, -1.0])
    }

    pub fn demod(&mut self, x0: f32) -> f32{
        let (lo1, lo0) = self.theta.sin_cos();
        let s0  = self.i_filter.filter(lo0 * x0);
        let s1  = self.q_filter.filter(lo1 * x0);
        let phi = self.loop_filt.filter(s1.atan2(s0));
        self.theta = (self.theta + self.w_lo + phi) % (2.0*PI as f32);
        self.matched.filter(s0)
    }
}
