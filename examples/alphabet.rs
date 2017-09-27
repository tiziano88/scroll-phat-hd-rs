extern crate scroll_phat_hd;

use scroll_phat_hd::display::*;
use scroll_phat_hd::scroller::*;

fn main() {
    println!("start");

    // let display = I2CDisplay::new(1);
    let display = TermDisplay::new();
    let mut scroller = Scroller::new(display);

    scroller.set_text("ABCDEFGHIJKLMNOPQRSTUVWXYZ 0123456789+=-_,.!?");
    for _ in 0..3000 {
        scroller.show();
        std::thread::sleep(std::time::Duration::from_millis(100));
        scroller.scroll();
    }

    println!("end");
}
