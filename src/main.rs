extern crate i2cdev;

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

struct Display {
    device: LinuxI2CDevice,
}

impl Display {
    fn new() -> Display {
        let d = LinuxI2CDevice::new("/dev/i2c-1", ADDRESS).unwrap();
        Display { device: d }
    }

    fn bank(&mut self, bank: u8) {
        self.device.smbus_write_block_data(BANK_ADDRESS, &[bank]).unwrap();
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

    fn test(&mut self) {
        // self.reset();
        self.bank(CONFIG_BANK);
        self.device.smbus_write_block_data(MODE_REGISTER, &[PICTURE_MODE]).unwrap();
        self.device.smbus_write_block_data(AUDIOSYNC_REGISTER, &[0]).unwrap();
        self.bank(1);
        self.device.smbus_write_block_data(0, &[255; 17]).unwrap();
        self.bank(0);
        self.device.smbus_write_block_data(0, &[255; 17]).unwrap();
        self.frame(0);

        self.bank(0);
        // self.device.smbus_write_byte_data(COLOR_OFFSET, 255).unwrap();
        self.device.smbus_write_block_data(COLOR_OFFSET, &[255]).unwrap();
        self.device.smbus_write_block_data(COLOR_OFFSET + 32, &[255; 32]).unwrap();
        self.device.smbus_write_block_data(COLOR_OFFSET + 64, &[255; 32]).unwrap();
        self.device.smbus_write_block_data(COLOR_OFFSET + 128, &[255; 32]).unwrap();
        self.frame(0);
    }
}

fn main() {
    println!("Hello, world!");
    let mut d = Display::new();
    d.test();
}
