//! # Scroll pHAT HD
//!
//! Sample usage:
//!
//! ```
//! let mut display = I2CDisplay::new(1);
//! let mut scroller = Scroller::new(&mut display);
//! scroller.set_text("ABC");
//! scroller.show();
//! ```

mod font;
mod shared;

pub mod display;
pub mod scroller;
