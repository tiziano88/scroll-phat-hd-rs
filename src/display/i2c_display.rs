#![allow(dead_code)]

extern crate i2cdev;

use super::*;

use self::i2cdev::core::I2CDevice;
use self::i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

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

/// A Scroll pHAT HD device connected over I2C bus (e.g. on a Raspberry Pi).
pub struct I2CDisplay {
    device: LinuxI2CDevice,
    frame: u8,
}

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

        display.init_display().unwrap();

        display
    }

    fn bank(&mut self, bank: u8) -> Result<(), LinuxI2CError> {
        self.write_data(BANK_ADDRESS, &[bank])
    }

    fn register(&mut self, bank: u8, register: u8, value: u8) -> Result<(), LinuxI2CError> {
        self.bank(bank)?;
        self.write_data(register, &[value])
    }

    fn frame(&mut self, frame: u8) -> Result<(), LinuxI2CError> {
        self.register(CONFIG_BANK, FRAME_REGISTER, frame)
    }

    fn write_data(&mut self, base_address: u8, data: &[u8]) -> Result<(), LinuxI2CError> {
        const CHUNK_SIZE: usize = 32;
        for (i, chunk) in data.chunks(CHUNK_SIZE).enumerate() {
            self.device
                .smbus_process_block(base_address + (i * CHUNK_SIZE) as u8, chunk)?;
        }
        Ok(())
    }

    fn reset_display(&mut self) -> Result<(), LinuxI2CError> {
        self.sleep(true)?;
        std::thread::sleep(std::time::Duration::from_millis(10));
        self.sleep(false)?;
        Ok(())
    }

    fn init_display(&mut self) -> Result<(), LinuxI2CError> {
        self.reset_display()?;

        // Switch to Picture Mode.
        self.register(CONFIG_BANK, MODE_REGISTER, PICTURE_MODE)?;

        // Disable audio sync.
        self.register(CONFIG_BANK, AUDIOSYNC_REGISTER, 0)?;

        // Initialize frames 0 and 1.
        for frame in 0..2 {
            self.write_data(BANK_ADDRESS, &[frame])?;

            // Turn off blinking for all LEDs.
            self.write_data(BLINK_OFFSET, &[0; LED_COLUMNS * LED_ROWS])?;

            // Set the PWM duty cycle for all LEDs to 0%.
            self.write_data(COLOR_OFFSET, &[0; LED_COLUMNS * LED_ROWS])?;

            // Turn all LEDs "on".
            self.write_data(ENABLE_OFFSET, &[127; LED_COLUMNS * LED_ROWS])?;
        }

        Ok(())
    }

    fn sleep(&mut self, value: bool) -> Result<(), LinuxI2CError> {
        self.register(CONFIG_BANK, SHUTDOWN_REGISTER, if value { 0 } else { 1 })
    }
}

impl Display for I2CDisplay {
    fn show(&mut self, buffer: &[Column]) -> Result<(), Error> {
        // Double buffering with frames 0 and 1.
        let new_frame = (self.frame + 1) % 2;
        self.bank(new_frame)?;
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
                self.write_data(COLOR_OFFSET + offset as u8, &[value])?;
            }
        }
        self.frame(new_frame)?;
        self.frame = new_frame;
        Ok(())
    }
}
