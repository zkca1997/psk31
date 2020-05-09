use std::error::Error;
use std::thread;
use std::time::Duration;
use std::convert::TryInto;

use rppal::gpio::Gpio;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use rppal::system::DeviceInfo;

const ADC0: &[u8] = &[ 0x01, 0x80, 0x00 ];
const STATUS_GPIO: u8 = 23; 

fn main() -> Result<(), Box<dyn Error>> {
    println!("{} Initializing Sampling", DeviceInfo::new()?.model());

    let mut status_pin = Gpio::new()?.get(STATUS_GPIO)?.into_output();
    status_pin.set_high();

    let clk_freq: u32 = 32_000;
    let spi = Spi::new( Bus::Spi0,
                        SlaveSelect::Ss0,
                        clk_freq,
                        Mode::Mode0 )?;

    println!("CLK freq: {}\nMode: {}", spi.clock_speed()?, spi.mode()?);
    
    let n: usize = 10;
    let mut read_buf: [u8; 3] = [0; 3];
    for _i in 0..n {
        spi.transfer( &mut read_buf, ADC0 )?;
        let x = u16::from_be_bytes(read_buf[1..3].try_into().unwrap()) & 0x03FF;
        let m = 3.3 * (x as f32) / 1023.0;
        println!("{:08b}|{:08b}|{:08b} -> {}V", read_buf[0], read_buf[1], read_buf[2], m); 
        thread::sleep(Duration::from_millis(500));
    }

    println!("Sampling Shutting Down");
    status_pin.set_low();
    Ok(())
}
