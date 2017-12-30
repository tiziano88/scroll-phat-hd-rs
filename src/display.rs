#![allow(dead_code)]

#[cfg(target_os = "linux")]
extern crate i2cdev;
extern crate termion;

#[cfg(target_os = "linux")]
use self::i2cdev::core::I2CDevice;
#[cfg(target_os = "linux")]
use self::i2cdev::linux::LinuxI2CDevice;

use std;
use shared::*;

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

/// Represents a device capable of displaying a rectangular bitmap buffer.
pub trait Display {
    fn show(&mut self, buffer: &[Column]);
}

#[cfg(target_os = "linux")]
/// A Scroll pHAT HD device connected over I2C bus (e.g. on a Raspberry Pi).
pub struct I2CDisplay {
    device: LinuxI2CDevice,
    frame: u8,
}

#[cfg(target_os = "linux")]
impl I2CDisplay {
    /// Creates a new I2CDisplay device using the I2C device identified by the provided
    /// `device_id` (normally 1 or 2).
    pub fn new(device_id: u8) -> I2CDisplay {
        let device_path = format!("/dev/i2c-{}", device_id);
        let d = LinuxI2CDevice::new(device_path, ADDRESS).unwrap();
        let mut display = I2CDisplay {
            device: d,
            frame: 0,
        };

        // Display initialization
        display.reset();

        display.device.smbus_write_byte_data(BANK_ADDRESS, CONFIG_BANK);
        // Switch to Picture Mode
        display.device.smbus_write_byte_data(MODE_REGISTER, PICTURE_MODE);
        // Disable audio sync
        display.device.smbus_write_byte_data(AUDIOSYNC_REGISTER, 0);

        // Initialize frame 1
        display.device.smbus_write_byte_data(BANK_ADDRESS, 1);
        // Turn off blinking for all LEDs
        for i in 0..17 {
            display.device.smbus_write_byte_data(BLINK_OFFSET + i, 0);
        }
        // Set the PWM duty cycle for all LEDs to 0%
        for i in 0..17 {
            for j in 0..7 {
                display.device.smbus_write_byte_data(COLOR_OFFSET + (i * 8) + j, 0);
            }
        }
        // Turn all LEDs "on"
        for i in 0..17 {
            display.device.smbus_write_byte_data(ENABLE_OFFSET + i, 127);
        }

        // Initialize frame 0
        display.device.smbus_write_byte_data(BANK_ADDRESS, 0);
        // Turn off blinking for all LEDs
        for i in 0..17 {
            display.device.smbus_write_byte_data(BLINK_OFFSET + i, 0);
        }
        // Set the PWM duty cycle for all LEDs to 0%
        for i in 0..17 {
            for j in 0..7 {
                display.device.smbus_write_byte_data(COLOR_OFFSET + (i * 8) + j, 0);
            }
        }
        // Turn all LEDs "on"
        for i in 0..17 {
            display.device.smbus_write_byte_data(ENABLE_OFFSET + i, 127);
        }

        display
    }

    fn bank(&mut self, bank: u8) -> Result<(), i2cdev::linux::LinuxI2CError> {
        self.device.smbus_write_byte_data(BANK_ADDRESS, bank)
    }

    fn register(
        &mut self,
        bank: u8,
        register: u8,
        value: u8,
    ) -> Result<(), i2cdev::linux::LinuxI2CError> {
        self.bank(bank);
        self.device.smbus_write_byte_data(register, value)
    }

    fn frame(&mut self, frame: u8) -> Result<(), i2cdev::linux::LinuxI2CError> {
        self.register(CONFIG_BANK, FRAME_REGISTER, frame)
    }

    fn reset(&mut self) {
        self.sleep(true);
        std::thread::sleep(std::time::Duration::from_millis(10));
        self.sleep(false);
    }

    fn sleep(&mut self, value: bool) -> Result<(), i2cdev::linux::LinuxI2CError> {
        self.register(CONFIG_BANK, SHUTDOWN_REGISTER, if value { 0 } else { 1 })
    }
}

#[cfg(target_os = "linux")]
impl Display for I2CDisplay {
    fn show(&mut self, buffer: &[Column]) {
        let new_frame = (self.frame + 1) % 2;
        self.bank(new_frame);
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
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
                    .smbus_write_byte_data(COLOR_OFFSET + offset as u8, value);
            }
        }
        self.frame(new_frame);
        self.frame = new_frame;
    }
}

/// A virtual display that outputs its buffer to the terminal from which the binary is attached.
///
/// Useful for debugging or prototyping, as it does not require a physical display to be connected.
pub struct TermDisplay {}

impl TermDisplay {
    pub fn new() -> TermDisplay {
        TermDisplay {}
    }
}

impl Display for TermDisplay {
    fn show(&mut self, buffer: &[Column]) {
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

/// A virtual display that outputs its buffer to the terminal from which the binary is attached.
///
/// Useful for debugging or prototyping, as it does not require a physical display to be connected.
pub struct UnicodeDisplay {}

impl UnicodeDisplay {
    pub fn new() -> UnicodeDisplay {
        UnicodeDisplay {}
    }
}

impl Display for UnicodeDisplay {
    fn show(&mut self, buffer: &[Column]) {
        print!("{}", termion::clear::All);

        let col = &buffer[0];

        println!("{}╔", termion::cursor::Goto(1, 1));
        for y in 0..col.len() {
            println!("{}║", termion::cursor::Goto(1, y as u16 + 2));
        }
        println!("{}╚", termion::cursor::Goto(1, col.len() as u16 + 2));

        for x in 0..buffer.len() {
            let col = &buffer[x];
            println!("{}═", termion::cursor::Goto(x as u16 + 2, 1));
            for y in 0..col.len() {
                let c = col[y];
                let v = if c == 0 { '░' } else { '▓' };
                println!("{}{}", termion::cursor::Goto(x as u16 + 2, y as u16 + 2), v);
            }
            println!(
                "{}═",
                termion::cursor::Goto(x as u16 + 2, col.len() as u16 + 2)
            );
        }

        println!("{}╗", termion::cursor::Goto(buffer.len() as u16 + 1, 1));
        for y in 0..col.len() {
            println!(
                "{}║",
                termion::cursor::Goto(buffer.len() as u16 + 1, y as u16 + 2)
            );
        }
        println!(
            "{}╝",
            termion::cursor::Goto(buffer.len() as u16 + 1, col.len() as u16 + 2)
        );
    }
}
