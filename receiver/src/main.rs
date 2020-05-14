extern crate ctrlc;
extern crate crossbeam_channel;
extern crate exitfailure;

mod costa;
mod filter;
//mod driver;
//mod sampler;

type Dsp = f32; // default type for internal signal

use std::thread;
use std::time::Duration;
use crossbeam_channel::{bounded, tick, Receiver, select};

const BAUD: u32 = 9600;             // UART baud rate
const DEV: &str = "/dev/ttyACM0";   // UART/USB device
const FS: f64   = 4e3;              // nominal sampling frequency
const FC: f64   = 4e2;              // nominal carrier frequency
const RB: f64   = 31.25;            // PSK31 baud rate

fn main() -> Result<(), exitfailure::ExitFailure> {

    // digital Costa loop carrier phase syncronizer / baseband demod
    let mut costa = costa::Costa::new(FS, FC, RB, 20.0);

    // spool channelized threads to manage workflow 
    let samp = sampler()?;              // collects samples from hardware
    let (tx, rx) = demodulator()?;      // collects demodulated characters
    let sig_event = ctrl_channel()?;    // collects control signals
    
    loop {
        select! {
            recv(samp) -> x => {
                tx.send(costa.sync(x));
            }
            recv(rx) -> symbol => {
                println!("{}", symbol);
            }
            recv(sig_event) -> _ => {
                println!("\nexiting receiver");
                break;
            }
        }
    }
    Ok(())
}

fn sampler() -> Result<Receiver<Dsp>, Uart::Error> {
    let (tx, rx) = unbounded();
    
    let usb_path = Path::new(DEV).as_ref();
    let mut uart = match Uart::with_path(usb_path, baud_rate, Parity::None, 8, 1)?;
    uart.set_read_mode(2, Duration::default())?
        .expect("failed to configure UART in blocking mode");
    uart.flush(Queue::Both);

    thread::spawn(move || loop {
        let mut buf = [0u8; 2];
        uart.read(&mut buf);
        let val = ((buf[0] as u16) << 8) | buf[1] as u16;
        tx.send(val);
    });
    Ok(rx)
}

fn demodulator() -> Result<(Sender<Dsp>, Receiver<Dsp>), ()> {
    let (tx, rx) = unbounded();

    let mut demod = demod::Demod::new();
    thread::spawn(move || loop { select! {
        recv(rx) -> x => {
            match demod.input(x) {
                Some(symbol) => tx.send(symbol),
                None         => (),
            }
        }
    }});
    Ok((tx, rx))
}

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (tx, rx) = bounded(1);
    ctrlc::set_handler(move || {
        let _ = tx.send(());
    })?;
    Ok(rx)
}
