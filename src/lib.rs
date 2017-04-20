mod font;
mod types;
mod projector;

pub use projector::*;
pub use types::*;

pub struct Display<'a> {
    scroll: usize,
    buffer: Vec<Column>,
    projector: &'a mut Projector,
}

impl<'a> Display<'a> {
    pub fn new(projector: &'a mut Projector) -> Display {
        Display {
            scroll: 0,
            buffer: vec![],
            projector: projector,
        }
    }

    pub fn scroll(&mut self) {
        self.scroll += 1;
        if self.scroll >= self.buffer.len() {
            self.scroll = 0;
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: u8) {
        if y >= DISPLAY_HEIGHT {
            return;
        }
        if x >= self.buffer.len() {
            for _ in self.buffer.len()..(x + 1) {
                self.buffer.push(EMPTY_COLUMN);
            }
        }
        self.buffer[x][y] = value;
    }

    pub fn set_text(&mut self, text: &str) {
        let font = font::font();
        for c in text.chars() {
            if let Some(glyph) = font.get(&c) {
                for c in glyph {
                    self.buffer.push(*c);
                }
                self.buffer.push(EMPTY_COLUMN);
            }
        }
    }

    pub fn show(&mut self) {
        let buffer: Vec<Column> =
            self.buffer.iter().skip(self.scroll).take(DISPLAY_WIDTH).cloned().collect();
        self.projector.project(&buffer);
    }
}
