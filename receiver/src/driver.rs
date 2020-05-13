use std::path::Path;
use std::io::{self, BufReader, BufRead};
use std::fs::File;
use std::convert::TryInto;

use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

type Dsp = f32;     // this is the type that the device reads / writes

pub struct MCP3008 {
    spi: Spi,
}

pub struct FileSource {
    reader: BufReader<File>,
}

impl MCP3008 {
    const ADC0: [u8; 3] = [ 0x01, 0x80, 0x00 ]; // command word to read channel 0
    const VREF: f32 = 3.3;                      // maximum allowable voltage to measure
    const RES:  f32 = 1023.0;                   // (2^n)-1 is the largest value readable

    pub fn new() -> Self {
        let clk_freq: u32 = 32_000;
        let spi = Spi::new( Bus::Spi0, SlaveSelect::Ss0, clk_freq, Mode::Mode0 ).expect("Failed to configure MCP3008");
        MCP3008 { spi }
    }

    pub fn poll(&self) -> Option<Dsp> {
        let mut read_buf: [u8; 3] = [0; 3];
        self.spi.transfer(&mut read_buf, &(MCP3008::ADC0)).expect("Failed to Read MCP3008");
        let val = u16::from_be_bytes(read_buf[1..3].try_into().unwrap()) & 0x03FF;
        Some(MCP3008::VREF * (val as Dsp) / MCP3008::RES)
    }
}

impl FileSource {
    pub fn new(filename: &'static str) -> Self {
        let path = Path::new(filename);
        let reader = match File::open(&path) {
            Err(_) => panic!("failed to open {}", path.display()),
            Ok(fid)  => BufReader::new(fid),
        };
        Self { reader }
    }
    
    pub fn poll(&mut self) -> Option<Dsp> {
        let mut buf = String::new();
        self.reader.read_line(&mut buf);
        match buf.parse::<Dsp>() {
            Ok(val) => Some(val),
            Err(_)  => None,
        }
    }
}
