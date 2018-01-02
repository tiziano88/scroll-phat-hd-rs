extern crate failure;
extern crate termion;

use self::failure::Error;

use std;
use shared::*;

const LED_COLUMNS: usize = 17;
const LED_ROWS: usize = 7;

/// Represents a device capable of displaying a rectangular bitmap buffer.
pub trait Display {
    fn show(&mut self, buffer: &[Column]) -> Result<(), Error>;
}

#[cfg(target_os = "linux")]
mod i2c_display;
#[cfg(target_os = "linux")]
pub use self::i2c_display::I2CDisplay;

mod term_display;
pub use self::term_display::TermDisplay;

mod unicode_display;
pub use self::unicode_display::UnicodeDisplay;
