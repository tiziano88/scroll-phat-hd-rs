extern crate rand;

mod font;
mod types;
mod projector;

use std::collections::HashMap;

use projector::*;
use types::*;

struct Display<'a> {
    scroll: usize,
    buffer: Vec<Column>,
    projector: &'a mut Projector,
}

impl<'a> Display<'a> {
    fn new(projector: &'a mut Projector) -> Display {
        Display {
            scroll: 0,
            buffer: vec![],
            projector: projector,
        }
    }

    fn scroll(&mut self) {
        self.scroll += 1;
        if self.scroll >= self.buffer.len() {
            self.scroll = 0;
        }
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
        let font = font::font();
        let brightness = 0x0F;
        let mut offset = 0;
        for c in text.chars() {
            if let Some(glyph) = font.get(&c) {
                for c in glyph {
                    self.buffer.push(*c);
                }
                self.buffer.push(EMPTY_COLUMN);
            }
        }
    }

    fn show(&mut self) {
        let buffer: Vec<Column> = self.buffer.iter().skip(self.scroll).take(17).cloned().collect();
        self.projector.project(&buffer);
    }
}

fn main() {
    println!("start");

    // let mut projector = I2CProjector::new();
    let mut projector = TermProjector::new();
    let mut d = Display::new(&mut projector);

    d.set_text("ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
    for _ in 0..3000 {
        d.show();
        std::thread::sleep(std::time::Duration::from_millis(100));
        d.scroll();
    }

    println!("end");
}
