extern crate scroll_phat_hd;

fn main() {
    println!("start");

    // let mut projector = I2CProjector::new();
    let mut projector = scroll_phat_hd::TermProjector::new();
    let mut d = scroll_phat_hd::Display::new(&mut projector);

    d.set_text("ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
    for _ in 0..3000 {
        d.show();
        std::thread::sleep(std::time::Duration::from_millis(100));
        d.scroll();
    }

    println!("end");
}
