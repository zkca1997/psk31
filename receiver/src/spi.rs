use std::thread;
use std::time::Duration;
use std::convert::TryInto;

use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

const ADC0: [u8; 3] = [ 0x01, 0x80, 0x00 ];  // command to read channel 0

pub struct MCP3008 {
    req: [u8; 3],
    spi: Spi,
}

impl MCP3008 {
    pub fn new() -> Self {
        let clk_freq: u32 = 32_000;
        let mut spi = Spi::new( Bus::Spi0, SlaveSelect::Ss0, clk_freq, Mode::Mode0 ).expect("Failed to configure MCP3008");
        MCP3008 { req: ADC0, spi }
    }

    pub fn poll(&self) -> f32 {
        let mut read_buf: [u8; 3] = [0; 3];
        self.spi.transfer(&mut read_buf, &(self.req)).expect("Failed to Read MCP3008");
        let val = u16::from_be_bytes(read_buf[1..3].try_into().unwrap()) & 0x03FF;
        3.3 * (val as f32) / 1023.0
    }
}
