use display::*;
use font::*;
use shared::*;

/// A virtual scrollable buffer with a scoll offset defining a visible window on the buffer itself.
/// It is composed by a horizontally growable virtual buffer, and an offset that defines what
/// portion of the buffer is actually visible.
///
/// ```text
/// ┌─────────────virtual buffer───────────┐
/// ┌───────╔═════════════════╗────────────┐
/// │▓▓▓▓▓▓▓║░░░░░░░░░░░░░░░░░║▓▓▓▓▓▓▓▓▓▓▓▓│
/// │▓▓▓▓▓▓▓║░░░░░░░░░░░░░░░░░║▓▓▓▓▓▓▓▓▓▓▓▓│
/// └───────╚═════════════════╝────────────┘
/// └offset─┴─visible buffer──┘
/// ```
pub struct Scroller<'a> {
    virtual_buffer: Vec<Column>,
    scroll_offset_x: usize,
    display: &'a mut Display,
}

impl<'a> Scroller<'a> {
    pub fn new(display: &'a mut Display) -> Scroller {
        Scroller {
            virtual_buffer: vec![],
            scroll_offset_x: 0,
            display: display,
        }
    }

    /// Moves the buffer one pixel to the right. When the window reaches the end of the virtual
    /// buffer, it starts again from the beginning.
    pub fn scroll(&mut self) {
        self.scroll_offset_x += 1;
        if self.scroll_offset_x >= self.virtual_buffer.len() {
            self.scroll_offset_x = 0;
        }
    }

    /// Sets the value of an individual pixel.
    pub fn set_pixel(&mut self, x: usize, y: usize, value: u8) {
        if y >= DISPLAY_HEIGHT {
            return;
        }
        if x >= self.virtual_buffer.len() {
            for _ in self.virtual_buffer.len()..(x + 1) {
                self.virtual_buffer.push(EMPTY_COLUMN);
            }
        }
        self.virtual_buffer[x][y] = value;
    }

    /// Sets the text of the buffer to the specified value.
    pub fn set_text(&mut self, text: &str) {
        self.clear();
        let font = font();
        for c in text.chars() {
            if let Some(glyph) = font.get(&c) {
                for c in glyph {
                    self.virtual_buffer.push(*c);
                }
                self.virtual_buffer.push(EMPTY_COLUMN);
            }
        }
    }

    fn visible_buffer(&self) -> Vec<Column> {
        self.virtual_buffer
            .iter()
            .skip(self.scroll_offset_x)
            .take(DISPLAY_WIDTH)
            .cloned()
            .collect()
    }

    /// Sends the visible buffer to the output display.
    pub fn show(&mut self) {
        let visible_buffer = self.visible_buffer();
        self.display.show(&visible_buffer);
    }

    /// Clears the buffer.
    pub fn clear(&mut self) {
        self.virtual_buffer = vec![];
    }
}
