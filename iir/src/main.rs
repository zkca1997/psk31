mod costa;
mod filter;

fn main() {
    let fs: f64 = 4e3;      // sampling frequency
    let fc: f64 = 4e2;      // carrier frequency
    let rb: f64 = 31.25;    // baud rate
    let mut costa = costa::Costa::new(fc, fs, rb, 0.05);
    let mut y: Vec<f32> = Vec::with_capacity(20);
    for _i in 0..20 {
        y.push( costa.demod(1.0) );
    }
    println!("{:?}", y);
}
