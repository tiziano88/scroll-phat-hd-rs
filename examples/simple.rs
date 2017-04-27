extern crate scroll_phat_hd;

fn main() {
    println!("start");

    // let mut projector = I2CProjector::new();
    let mut display = scroll_phat_hd::TermDisplay::new();
    let mut scroller = scroll_phat_hd::Scroller::new(&mut display);

    scroller.set_text("ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
    for _ in 0..3000 {
        scroller.show();
        std::thread::sleep(std::time::Duration::from_millis(100));
        scroller.scroll();
    }

    println!("end");
}
