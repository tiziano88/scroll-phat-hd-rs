//! # Scroll pHAT HD
//!
//! Sample usage:
//!
//! ```
//! let display = scroll_phat_hd::display::TermDisplay::new();
//! let mut scroller = scroll_phat_hd::scroller::Scroller::new(display);
//! scroller.set_text("ABC");
//! scroller.show();
//! ```

mod font;
mod shared;

pub mod display;
pub mod scroller;
