mod costa;
mod filter;
mod spi;

const Fs: f64 = 4e3;        // sampling frequency
const Fc: f64 = 4e2;        // carrier frequency
const Rb: f64 = 31.25;      // baud rate

fn main() {
    let source = spi::MCP3008::new();
    let mut costa = costa::Costa::new(Fc, Fs, Rb, 20.0);

    let mut y: Vec<f32> = Vec::with_capacity(20);
    for _i in 0..20 {
        let x = source.poll();
        println!("{}", x);
        y.push( costa.demod(1.0) );
    }
}
