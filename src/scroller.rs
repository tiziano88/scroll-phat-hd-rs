use display::*;
use font::*;
use shared::*;

/// A virtual scrollable buffer with a scoll offset defining a visible window on the buffer itself.
pub struct Scroller<'a> {
    scroll: usize,
    buffer: Vec<Column>,
    display: &'a mut Display,
}

impl<'a> Scroller<'a> {
    pub fn new(display: &'a mut Display) -> Scroller {
        Scroller {
            scroll: 0,
            buffer: vec![],
            display: display,
        }
    }

    /// Moves the buffer one pixel to the right.
    pub fn scroll(&mut self) {
        self.scroll += 1;
        if self.scroll >= self.buffer.len() {
            self.scroll = 0;
        }
    }

    /// Sets the value of an individual pixel.
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

    /// Sets the text of the buffer to the specified value.
    pub fn set_text(&mut self, text: &str) {
        self.clear();
        let font = font();
        for c in text.chars() {
            if let Some(glyph) = font.get(&c) {
                for c in glyph {
                    self.buffer.push(*c);
                }
                self.buffer.push(EMPTY_COLUMN);
            }
        }
    }

    /// Sends the buffer to the output display.
    pub fn show(&mut self) {
        let buffer: Vec<Column> =
            self.buffer.iter().skip(self.scroll).take(DISPLAY_WIDTH).cloned().collect();
        self.display.show(&buffer);
    }

    /// Clears the buffer.
    pub fn clear(&mut self) {
        self.buffer = vec![];
    }
}
