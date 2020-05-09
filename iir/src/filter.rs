extern crate num;

use std::collections::VecDeque;
use num::complex::Complex;
use std::fmt;
use std::f64;

const PI: f64 = f64::consts::PI;
type Cplx = Complex<f64>;

pub struct IIRFilter {
    a: Vec<f32>,
    b: Vec<f32>,
    x: VecDeque<f32>,
    y: VecDeque<f32>,
}

pub struct FIRFilter {
    b: Vec<f32>,
    x: VecDeque<f32>,
}

impl IIRFilter {
    pub fn new(b: &[f32], a: &[f32]) -> Self {
        let b: Vec<f32> = b.to_vec();
        let a: Vec<f32> = a[1..].to_vec();                                      // drop y[n] constant
        let x: VecDeque<f32> = VecDeque::from(vec![0.0; b.len()]);              // filter input history buffer
        let y: VecDeque<f32> = VecDeque::from(vec![0.0; a.len()]);              // filter output history buffer
        IIRFilter { a, b, x, y }
    }

    pub fn butterworth(n: usize, fc: f64, fs: f64) -> Self {
        let g = Complex::new(1.0/(PI*fc/fs).tan(), 0.0);                        // scaling value for analog=>digital cutoff freq
        let p = (0..n).map(|k| (PI*((2*k+1) as f64)/((2*n) as f64)).sin_cos())  // theta of butterworth poles
                      .map(|(s, c)| Complex::new( -1.0*s, c ))                  // normalized butterworth poles
                      .map(|p| (g + p) / (g - p))                               // bilinear transform & Fc scaling
                      .collect::<Vec<Cplx>>();
        let z = vec![-0.99; n];                                                 // n # of zeros just inside z-domain unit circle
        
        let b = z.poly();                                                       // poly coefs of num (zeros)
        let a = p.poly();                                                       // poly coefs of denom (poles)
        let k = a.iter().sum::<f64>() / b.iter().sum::<f64>();                  // normalization constant for passband 0dB gain
        let b: Vec<f32> = b.iter().map(|&x| (k * x) as f32).collect();          // apply normalization constant
        let a: Vec<f32> = a.iter().map(|&x| x as f32).collect();
        IIRFilter::new(b.as_slice(), a.as_slice())
    }

    pub fn filter(&mut self, x0: f32) -> f32 {
        self.x.pop_back();
        self.x.push_front(x0);                                                  // update input data buffer
        let t = self.x.iter().zip(self.b.iter())                                // sum: b[0..] * x[n..]
                    .fold(0.0, |acc, (b, x)| acc + b * x);
        let y0 = self.y.iter().zip(self.a.iter())                               // sum: t + (a[0..] * y[n-1..])
                     .fold(t, |acc, (a, y)| acc - a * y);
        self.y.pop_back();
        self.y.push_front(y0);                                                  // update output data buffer
        y0
    }
}

impl FIRFilter {
    pub fn new(b: &[f32]) -> Self {
        let b: Vec<f32> = b.to_vec();                                           // b coefficients
        let x: VecDeque<f32> = VecDeque::from(vec![0.0; b.len()]);              // filter input history buffer
        FIRFilter { b, x }
    }

    // generates root-raised cosine pulse forming filter with beta = 0.5 spanning one symbol
    pub fn pulse_shaper(n: i32) -> Self {
        assert!(n % 2 == 0 && n > 0, "pulse forming filter must have an even order");
        let mut b: Vec<f64> = Vec::with_capacity((n+1) as usize);
        for i in 0..=n {
            let k = ((2*i-n) as f64) / ((2*n) as f64);
            let nf = n as f64;

            if i == 0 || i == n {
                b.push((1.5*PI*(0.75*PI).sin() - 2.0*(-0.25*PI).sin() - 0.5*PI*(-0.25*PI).cos()) / (2.0*PI*nf));
            } else if i == (n/2) {
                b.push((4.0 + PI) / (2.0 * PI * (nf)));
            } else {
                b.push(((1.5*PI*k).cos() + ((0.5*PI*k).sin()/(2.0*k))) / (-0.5*nf*PI*(4.0*k*k - 1.0)));
            }
        }
        let k = b.iter().sum::<f64>();
        let b: Vec<f32> = b.iter().map(|x| (x / k) as f32).collect();
        FIRFilter::new(b.as_slice())
    }

    pub fn filter(&mut self, x0: f32) -> f32 {
        self.x.pop_back();
        self.x.push_front(x0);                                                  // update input data buffer
        self.x.iter().zip(self.b.iter())
            .fold(0.0, |acc, (b, x)| acc + b * x)
    }
}

impl fmt::Display for IIRFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "b = ")?;
        for c in self.b.iter() { write!(f, "{:e}, ", c)?; }
        write!(f, "\na = ")?;
        for c in self.a.iter() { write!(f, "{:e}, ", c)?; }
        Ok(())
    }
}

impl fmt::Display for FIRFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "b = ")?;
        for c in self.b.iter() { write!(f, "{:e}, ", c)?; }
        Ok(())
    }
}

trait Poly<T> {
    fn poly(&self) -> Vec<f64>;
}

impl Poly<Cplx> for Vec<Cplx> {
    fn poly(&self) -> Vec<f64> {
        let mut p = vec![Complex::new(0.0, 0.0); self.len()+1];
        p[0] = Complex::new(1.0, 0.0);
        for i in 0..self.len() {
            for j in (1..(i+2)).rev() {
                p[j] = p[j] - self[i] * p[j-1];
            }
        }
        p.iter().map(|x| x.re).collect::<Vec<f64>>()
    }
}

impl Poly<f64> for Vec<f64> {
    fn poly(&self) -> Vec<f64> {
        let mut p = vec![0.0; self.len()+1];
        p[0] = 1.0;
        for i in 0..self.len() {
            for j in (1..(i+2)).rev() {
                p[j] = p[j] - self[i] * p[j-1];
            }
        }
        p
    }
}
