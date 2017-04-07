extern crate i2cdev;
extern crate rand;

use i2cdev::core::I2CDevice;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

const MODE_REGISTER: u8 = 0x00;
const FRAME_REGISTER: u8 = 0x01;
const AUTOPLAY1_REGISTER: u8 = 0x02;
const AUTOPLAY2_REGISTER: u8 = 0x03;
const BLINK_REGISTER: u8 = 0x05;
const AUDIOSYNC_REGISTER: u8 = 0x06;
const BREATH1_REGISTER: u8 = 0x08;
const BREATH2_REGISTER: u8 = 0x09;
const SHUTDOWN_REGISTER: u8 = 0x0a;
const GAIN_REGISTER: u8 = 0x0b;
const ADC_REGISTER: u8 = 0x0c;

const CONFIG_BANK: u8 = 0x0b;
const BANK_ADDRESS: u8 = 0xfd;

const PICTURE_MODE: u8 = 0x00;
const AUTOPLAY_MODE: u8 = 0x08;
const AUDIOPLAY_MODE: u8 = 0x18;

const ENABLE_OFFSET: u8 = 0x00;
const BLINK_OFFSET: u8 = 0x12;
const COLOR_OFFSET: u8 = 0x24;

const ADDRESS: u16 = 0x74;

#[cfg_attr(rustfmt, rustfmt_skip)]
const a: [&str; 7] = [
"   ",
"   ",
"xxx",
"  x",
"xxx",
"x x",
"xxx"];

#[cfg_attr(rustfmt, rustfmt_skip)]
const b: [&str; 7] = [
"x  ",
"x  ",
"x  ",
"x  ",
"xxx",
"x x",
"xxx"];

#[cfg_attr(rustfmt, rustfmt_skip)]
const c: [&str; 7] = [
"   ",
"   ",
"   ",
"   ",
"xxx",
"x  ",
"xxx"];

struct Display {
    device: LinuxI2CDevice,
    scroll: usize,
    buffer: [[u8; 17]; 7],
}

impl Display {
    fn new() -> Display {
        let d = LinuxI2CDevice::new("/dev/i2c-1", ADDRESS).unwrap();
        Display {
            device: d,
            scroll: 0,
            buffer: [[0; 17]; 7],
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

    fn set_pixel(&mut self, x: usize, y: usize, value: u8) {
        self.buffer[y][x] = value;
    }

    fn show(&mut self) {
        for y in 0..7 {
            for x in 0..17 {
                let offset = if x >= 8 {
                    (x - 8) * 16 + y
                } else {
                    (8 - x) * 16 - (y + 2)
                };
                let value = self.buffer[y as usize][x as usize];
                self.device
                    .smbus_write_byte_data(COLOR_OFFSET + offset, value)
                    .unwrap();
            }
        }
    }

    fn test(&mut self) {
        self.register(CONFIG_BANK, MODE_REGISTER, PICTURE_MODE);

        let o = ["xxxxx xxxxx x   x",
                 "  x       x x   x",
                 "  x      x  xx  x",
                 "  x     x   x x x",
                 "  x    x    x  xx",
                 "  x   x     x   x",
                 "  x   xxxxx x   x"];

        self.bank(0);
        for y in 0..7 {
            for x in 0..17 {
                self.set_pixel(x,
                               y,
                               if o[y as usize].chars().nth(x as usize).unwrap() == ' ' {
                                   0x00
                               } else {
                                   0x0F
                               });
            }
        }
        self.show();
    }
}

fn main() {
    println!("start");
    let mut d = Display::new();
    d.test();
    println!("end");
}
