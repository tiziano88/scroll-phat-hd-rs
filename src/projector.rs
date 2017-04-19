extern crate i2cdev;
extern crate termion;

use self::i2cdev::core::I2CDevice;
use self::i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

use std;
use types::*;

const MODE_REGISTER: u8 = 0x00;
const FRAME_REGISTER: u8 = 0x01;
const AUTOPLAY1_REGISTER: u8 = 0x02;
const AUTOPLAY2_REGISTER: u8 = 0x03;
const BLINK_REGISTER: u8 = 0x05;
const AUDIOSYNC_REGISTER: u8 = 0x06;
const BREATH1_REGISTER: u8 = 0x08;
const BREATH2_REGISTER: u8 = 0x09;
const SHUTDOWN_REGISTER: u8 = 0x0A;
const GAIN_REGISTER: u8 = 0x0B;
const ADC_REGISTER: u8 = 0x0C;

const CONFIG_BANK: u8 = 0x0B;
const BANK_ADDRESS: u8 = 0xFD;

const PICTURE_MODE: u8 = 0x00;
const AUTOPLAY_MODE: u8 = 0x08;
const AUDIOPLAY_MODE: u8 = 0x18;

const ENABLE_OFFSET: u8 = 0x00;
const BLINK_OFFSET: u8 = 0x12;
const COLOR_OFFSET: u8 = 0x24;

const ADDRESS: u16 = 0x74;

pub trait Projector {
    fn project(&mut self, &[Column]);
}

pub struct I2CProjector {
    device: LinuxI2CDevice,
    frame: u8,
    brightness: u8,
}

impl I2CProjector {
    pub fn new() -> I2CProjector {
        let d = LinuxI2CDevice::new("/dev/i2c-1", ADDRESS).unwrap();
        // self.register(CONFIG_BANK, MODE_REGISTER, PICTURE_MODE);
        I2CProjector {
            device: d,
            frame: 0,
            brightness: 0x0F,
        }
    }

    fn bank(&mut self, bank: u8) {
        self.device.smbus_write_byte_data(BANK_ADDRESS, bank).unwrap();
    }

    fn register(&mut self, bank: u8, register: u8, value: u8) {
        self.bank(bank);
        self.device.smbus_write_block_data(register, &[value]).unwrap();
    }

    fn frame(&mut self, frame: u8) {
        self.register(CONFIG_BANK, FRAME_REGISTER, frame);
    }

    fn reset(&mut self) {
        self.sleep(true);
        std::thread::sleep(std::time::Duration::from_millis(10));
        self.sleep(false);
    }

    fn sleep(&mut self, value: bool) {
        self.register(CONFIG_BANK, SHUTDOWN_REGISTER, if value { 0 } else { 1 });
    }
}

impl Projector for I2CProjector {
    fn project(&mut self, buffer: &[Column]) {
        // TODO(tzn): Double buffering.
        // let new_frame = (self.frame + 1) % 2;
        let new_frame = 1;
        self.bank(new_frame);
        for y in 0..7 {
            for x in 0..17 {
                let offset = if x >= 8 {
                    (x - 8) * 16 + y
                } else {
                    (8 - x) * 16 - (y + 2)
                };
                let value = match buffer.get(x as usize) {
                    Some(column) => column[y as usize],
                    None => 0,
                };
                self.device
                    .smbus_write_byte_data(COLOR_OFFSET + offset, value)
                    .unwrap();
            }
        }
        self.frame(new_frame);
        self.frame = new_frame;
    }
}

pub struct TermProjector {}

impl TermProjector {
    pub fn new() -> TermProjector {
        TermProjector {}
    }
}

impl Projector for TermProjector {
    fn project(&mut self, buffer: &[Column]) {
        print!("{}", termion::clear::All);
        for x in 0..buffer.len() {
            let col = &buffer[x];
            for y in 0..col.len() {
                let c = col[y];
                let v = if c == 0 { ' ' } else { '#' };
                println!("{}{}", termion::cursor::Goto(x as u16 + 1, y as u16 + 1), v);
            }
        }
    }
}
