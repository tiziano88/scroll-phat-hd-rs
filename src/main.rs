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

type Column = [u8; 7];
type Glyph = Vec<&'static Column>;

const EMPTY_COLUMN: Column = [0; 7];

fn make_glyph(v: [&'static str; 7]) -> Glyph {
    let width = v[0].len();
    let mut glyph = vec![];
    for _ in 0..width {
        glyph.push(EMPTY_COLUMN);
    }
    for y in 0..v.len() {
        let row = v[y];
        for x in 0..row.len() {
            let c = row.chars().nth(x).unwrap();
            glyph[x][y] = if c == ' ' { 0x00 } else { 0x0F };
        }
    }
    glyph
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn font() -> HashMap<char, Glyph> {
    let mut glyphs = HashMap::new();
    glyphs.insert('0', make_glyph([
                  "    " ,
                  " xx ",
                  "x  x",
                  "x xx",
                  "xx x",
                  "x  x",
                  " xx "]));
    glyphs.insert('1', make_glyph([
                  "   " ,
                  " x ",
                  "xx ",
                  " x ",
                  " x ",
                  " x ",
                  "xxx"]));
    glyphs.insert('2', make_glyph([
                  "    " ,
                  "xxx ",
                  "   x",
                  "  x ",
                  " x  ",
                  "x   ",
                  "xxxx"]));
    glyphs.insert('3', make_glyph([
                  "    " ,
                  "xxx ",
                  "   x",
                  " xx ",
                  "   x",
                  "   x",
                  "xxx "]));
    glyphs.insert('4', make_glyph([
                  "    " ,
                  "   x",
                  "  x ",
                  " x  ",
                  "x  x",
                  "xxxx",
                  "   x"]));
    glyphs.insert('5', make_glyph([
                  "    " ,
                  "xxxx",
                  "x   ",
                  "xxx ",
                  "   x",
                  "   x",
                  "xxx "]));
    glyphs.insert('6', make_glyph([
                  "    " ,
                  " xxx",
                  "x   ",
                  "xxx ",
                  "x  x",
                  "x  x",
                  " xx "]));
    glyphs.insert('7', make_glyph([
                  "    " ,
                  "xxxx",
                  "   x",
                  "  x ",
                  " x  ",
                  "x   ",
                  "x   "]));
    glyphs.insert('8', make_glyph([
                  "    " ,
                  " xx ",
                  "x  x",
                  " xx ",
                  "x  x",
                  "x  x",
                  " xx "]));
    glyphs.insert('9', make_glyph([
                  "    " ,
                  " xx ",
                  "x  x",
                  " xxx",
                  "   x",
                  "   x",
                  " xx "]));
    glyphs.insert('A', make_glyph([
                  "    " ,
                  " xx ",
                  "x  x",
                  "x  x",
                  "xxxx",
                  "x  x",
                  "x  x"]));
    glyphs.insert('B', make_glyph([
                  "    " ,
                  "xxx ",
                  "x  x",
                  "xxx ",
                  "x  x",
                  "x  x",
                  "xxx "]));
    glyphs.insert('C', make_glyph([
                  "    " ,
                  " xxx",
                  "x   ",
                  "x   ",
                  "x   ",
                  "x   ",
                  " xxx"]));
    glyphs.insert('D', make_glyph([
                  "    " ,
                  "xxx ",
                  "x  x",
                  "x  x",
                  "x  x",
                  "x  x",
                  "xxx "]));
    glyphs.insert('E', make_glyph([
                  "    " ,
                  "xxxx",
                  "x   ",
                  "xxx ",
                  "x   ",
                  "x   ",
                  "xxxx"]));
    glyphs.insert('F', make_glyph([
                  "    " ,
                  "xxxx",
                  "x   ",
                  "xxx ",
                  "x   ",
                  "x   ",
                  "x   "]));
    glyphs.insert('G', make_glyph([
                  "    " ,
                  " xxx",
                  "x   ",
                  "x   ",
                  "x xx",
                  "x  x",
                  " xxx"]));
    glyphs.insert('H', make_glyph([
                  "    " ,
                  "x  x",
                  "x  x",
                  "xxxx",
                  "x  x",
                  "x  x",
                  "x  x"]));
    glyphs.insert('I', make_glyph([
                  " " ,
                  "x",
                  "x",
                  "x",
                  "x",
                  "x",
                  "x"]));
    glyphs.insert('J', make_glyph([
                  "    " ,
                  "   x",
                  "   x",
                  "   x",
                  "   x",
                  "x  x",
                  " xx "]));
    glyphs.insert('K', make_glyph([
                  "    " ,
                  "x  x",
                  "x x ",
                  "xx  ",
                  "x x ",
                  "x  x",
                  "x  x"]));
    glyphs.insert('L', make_glyph([
                  "   " ,
                  "x  ",
                  "x  ",
                  "x  ",
                  "x  ",
                  "x  ",
                  "xxx"]));
    glyphs.insert('M', make_glyph([
                  "     " ,
                  "x   x",
                  "xx xx",
                  "x x x",
                  "x   x",
                  "x   x",
                  "x   x"]));
    glyphs.insert('N', make_glyph([
                  "    " ,
                  "x  x",
                  "xx x",
                  "x xx",
                  "x  x",
                  "x  x",
                  "x  x"]));
    glyphs.insert('O', make_glyph([
                  "    " ,
                  " xx ",
                  "x  x",
                  "x  x",
                  "x  x",
                  "x  x",
                  " xx "]));
    glyphs.insert('P', make_glyph([
                  "    " ,
                  "xxx ",
                  "x  x",
                  "xxx ",
                  "x   ",
                  "x   ",
                  "x   "]));
    glyphs.insert('Q', make_glyph([
                  "     " ,
                  " xx  ",
                  "x  x ",
                  "x  x ",
                  "x  x ",
                  "x xx ",
                  " xx x"]));
    glyphs.insert('R', make_glyph([
                  "     " ,
                  "xxx  ",
                  "x  x ",
                  "xxx  ",
                  "x  x ",
                  "x  x ",
                  "x  x "]));
    glyphs.insert('S', make_glyph([
                  "    " ,
                  " xxx",
                  "x   ",
                  " xx ",
                  "   x",
                  "   x",
                  "xxx "]));
    glyphs.insert('T', make_glyph([
                  "     " ,
                  "xxxxx",
                  "  x  ",
                  "  x  ",
                  "  x  ",
                  "  x  ",
                  "  x  "]));
    glyphs.insert('U', make_glyph([
                  "    " ,
                  "x  x",
                  "x  x",
                  "x  x",
                  "x  x",
                  "x  x",
                  " xx "]));
    glyphs.insert('V', make_glyph([
                  "     " ,
                  "x   x",
                  "x   x",
                  "x   x",
                  "x   x",
                  " x x ",
                  "  x  "]));
    glyphs.insert('W', make_glyph([
                  "     " ,
                  "x   x",
                  "x   x",
                  "x   x",
                  "x x x",
                  "x x x",
                  " x x "]));
    glyphs.insert('X', make_glyph([
                  "     " ,
                  "x   x",
                  " x x ",
                  "  x  ",
                  " x x ",
                  "x   x",
                  "x   x"]));
    glyphs.insert('Y', make_glyph([
                  "     " ,
                  "x   x",
                  " x x ",
                  "  x  ",
                  "  x  ",
                  "  x  ",
                  "  x  "]));
    glyphs.insert('Z', make_glyph([
                  "    " ,
                  "xxxx",
                  "   x",
                  "  x ",
                  " x  ",
                  "x   ",
                  "xxxx"]));
    glyphs
}

struct Display {
    device: LinuxI2CDevice,
    scroll: usize,
    buffer: Vec<&'static Column>,
    frame: u8,
    brightness: u8,
}

impl Display {
    fn new() -> Display {
        let d = LinuxI2CDevice::new("/dev/i2c-1", ADDRESS).unwrap();
        Display {
            device: d,
            scroll: 0,
            buffer: vec![EMPTY_COLUMN],
            frame: 0,
            brightness: 0x0F,
        }
    }

    fn bank(&mut self, bank: u8) {
        self.device.smbus_write_byte_data(BANK_ADDRESS, bank).unwrap();
    }

    fn scroll(&mut self) {
        self.scroll += 1;
        if self.scroll >= self.buffer.len() {
            self.scroll = 0;
        }
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
        if y >= 7 {
            return;
        }
        if x >= self.buffer.len() {
            for _ in self.buffer.len()..(x + 1) {
                self.buffer.push(EMPTY_COLUMN);
            }
        }
        self.buffer[x][y] = value;
    }

    fn set_text(&mut self, text: &str) {
        let font = font();
        let brightness = self.brightness;
        let mut offset = 0;
        for c in text.chars() {
            if let Some(glyph) = font.get(&c) {
                self.buffer.append(glyph);
                self.buffer.push(EMPTY_COLUMN);
            }
        }
    }

    fn show(&mut self) {
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
                let value = match self.buffer.get(self.scroll + x as usize) {
                    Some(column) => column[y as usize],
                    None => 0,
                };
                match self.device.smbus_write_byte_data(COLOR_OFFSET + offset, value) {
                    Ok(_) => {}
                    Err(err) => println!("error writing to i2c device: {}", err),
                };
            }
        }
        self.frame(new_frame);
        self.frame = new_frame;
    }

    fn test(&mut self) {
        self.register(CONFIG_BANK, MODE_REGISTER, PICTURE_MODE);
        self.set_text("ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");

        for _ in 0..3000 {
            self.show();
            std::thread::sleep(std::time::Duration::from_millis(100));
            self.scroll();
        }
    }
}

fn main() {
    font();
    println!("start");
    let mut d = Display::new();
    d.test();
    println!("end");
}
