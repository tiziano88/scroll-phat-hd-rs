extern crate i2cdev;
extern crate rand;

use std::collections::HashMap;
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

type Glyph = [&'static str; 7];

#[cfg_attr(rustfmt, rustfmt_skip)]
fn font() -> HashMap<char, Glyph> {
    let mut glyphs = HashMap::new();
    glyphs.insert('A', [
                  "    " ,
                  " xx ",
                  "x  x",
                  "x  x",
                  "xxxx",
                  "x  x",
                  "x  x"]);
    //glyphs.insert('A', [
                  //"xxxxx  ",
                  //"  x  x ",
                  //"     x ",
                  //"xxxxx  "]);
    glyphs.insert('B', [
                  "    " ,
                  "xxx ",
                  "x  x",
                  "xxx ",
                  "x  x",
                  "x  x",
                  "xxx "]);
    glyphs.insert('C', [
                  "    " ,
                  " xxx",
                  "x   ",
                  "x   ",
                  "x   ",
                  "x   ",
                  " xxx"]);
    glyphs.insert('D', [
                  "    " ,
                  "xxx ",
                  "x  x",
                  "x  x",
                  "x  x",
                  "x  x",
                  "xxx "]);
    glyphs.insert('E', [
                  "    " ,
                  "xxxx",
                  "x   ",
                  "xxx ",
                  "x   ",
                  "x   ",
                  "xxxx"]);
    glyphs.insert('F', [
                  "    " ,
                  "xxxx",
                  "x   ",
                  "xxx ",
                  "x   ",
                  "x   ",
                  "x   "]);
    glyphs.insert('G', [
                  "    " ,
                  " xxx",
                  "x   ",
                  "x   ",
                  "x xx",
                  "x  x",
                  " xxx"]);
    glyphs
}

struct Display {
    device: LinuxI2CDevice,
    scroll: usize,
    buffer: [[u8; 17]; 7],
    buffer1: Vec<[u8; 7]>,
}

impl Display {
    fn new() -> Display {
        let d = LinuxI2CDevice::new("/dev/i2c-1", ADDRESS).unwrap();
        Display {
            device: d,
            scroll: 0,
            buffer: [[0; 17]; 7],
            buffer1: vec![],
        }
    }

    fn bank(&mut self, bank: u8) {
        self.device.smbus_write_byte_data(BANK_ADDRESS, bank).unwrap();
    }

    fn scroll(&mut self) {
        self.scroll += 1;
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

    fn set_text(&mut self, text: &str) {
        let font = font();
        let mut offset = 0;
        for c in text.chars() {
            let glyph = font[&c];
            // self.buffer1.extend(glyph);
            for y in 0..glyph.len() {
                let row = glyph[y];
                for x in 0..row.len() {
                    let pixel = row.chars().nth(x).unwrap();
                    self.set_pixel(x + offset, y, if pixel == ' ' { 0x00 } else { 0x0F });
                }
            }
            // We assume that all the rows have equal length.
            offset += glyph[0].len() + 1;
        }
    }

    fn show(&mut self) {
        for y in 0..7 {
            for x in 0..17 {
                let offset = if x >= 8 {
                    (x - 8) * 16 + y
                } else {
                    (8 - x) * 16 - (y + 2)
                };
                let value = self.buffer[y as usize][self.scroll + x as usize];
                self.device
                    .smbus_write_byte_data(COLOR_OFFSET + offset, value)
                    .unwrap();
            }
        }
    }

    fn test(&mut self) {
        self.register(CONFIG_BANK, MODE_REGISTER, PICTURE_MODE);
        self.bank(0);
        self.set_text("ABC");

        self.show();
        // for _ in 0..10 {
        // std::thread::sleep(std::time::Duration::from_millis(100));
        // self.scroll();
        // }
    }
}

fn main() {
    font();
    println!("start");
    let mut d = Display::new();
    d.test();
    println!("end");
}
