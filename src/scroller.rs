extern crate rusttype;

use display::*;
use shared::*;

/// A virtual scrollable buffer with a scoll offset defining a visible window on the buffer itself.
/// It is composed by a horizontally growable virtual buffer, and an offset that defines what
/// portion of the buffer is actually visible.
///
/// ```text
/// ┌─────────────────────────virtual buffer─────────────────────┐
/// ┌───────────╔═════════════════╗──────────────────────────────┐
/// │▓▓▓▓▓▓▓▓▓▓▓║░░░░░░░░░░░░░░░░░║▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│
/// │▓▓▓▓▓▓▓▓▓▓▓║░░░░░░░░░░░░░░░░░║▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│
/// │▓▓▓▓▓▓▓▓▓▓▓║░░░░░░░░░░░░░░░░░║▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓│
/// └───────────╚═════════════════╝──────────────────────────────┘
/// └──offset───┴─visible buffer──┘
/// ```
pub struct Scroller<'a> {
    virtual_buffer: Vec<Column>,
    scroll_offset: usize,
    display: &'a mut Display,
    font: rusttype::Font<'a>,
    font_scale: f32,
}

impl<'a> Scroller<'a> {
    /// Create a new `Scroller` using the provided `Display`.
    pub fn new(display: &'a mut Display) -> Scroller<'a> {
        // Scale values for other fonts found on https://www.dafont.com/:
        // Minecraftia-Regular: 11.0
        // 8-bit pusab: 10.0
        // Pixeled: 14.0
        // Commodore Pixelized v1.2: 10.0
        // 000webfont: ???
        // aerxtabs_memesbruh03: ???
        // slkscr: ???
        // visitor1: ???
        let font_data = include_bytes!("./../fonts/Commodore Pixelized v1.2.ttf");
        let collection = rusttype::FontCollection::from_bytes(font_data as &[u8]);
        let font = collection.into_font().unwrap();
        Scroller {
            virtual_buffer: vec![],
            scroll_offset: 0,
            display: display,
            font: font,
            font_scale: 10.0,
        }
    }

    /// Moves the visible buffer one pixel to the right. When the right side of the visible buffer
    /// reaches the right side of the virtual buffer, it starts again from the beginning.
    pub fn scroll(&mut self) {
        self.scroll_offset += 1;
        if self.scroll_offset >= self.virtual_buffer.len() {
            self.scroll_offset = 0;
        }
    }

    /// Sets the value of an individual pixel in the virtual buffer.
    ///
    /// Coordinates are zero-based, with the origin in the top left corner of the virtual buffer,
    /// and increasing down and to the right.
    ///
    /// If the virtual buffer is not wide enough to contain the pixel, it is automatically extended
    /// up to that size.
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

    /// Clears the virtual buffer and sets the text to the specified value.
    pub fn set_text(&mut self, text: &str) {
        self.clear();
        let mut start = 0usize;
        for g in self.font.clone().glyphs_for(text.chars()) {
            let scaled_glyph = g.scaled(rusttype::Scale::uniform(self.font_scale));
            let width = scaled_glyph.h_metrics().advance_width as usize;
            let positioned_glyph = scaled_glyph.positioned(rusttype::point(0.0, 0.0));
            positioned_glyph.draw(|x, y, v| {
                let x = x as usize;
                let y = y as usize;
                let v = (v * 255.0) as u8;
                self.set_pixel(start + x, y, v);
            });
            start += width;
        }
    }

    /// Returns the visible buffer taking into account the current scroll offset.
    fn visible_buffer(&self) -> Vec<Column> {
        self.virtual_buffer
            .iter()
            .skip(self.scroll_offset)
            .take(DISPLAY_WIDTH)
            .cloned()
            .collect()
    }

    /// Sends the visible buffer to the output display.
    pub fn show(&mut self) {
        let visible_buffer = self.visible_buffer();
        self.display.show(&visible_buffer).unwrap();
    }

    /// Clears the virtual buffer.
    pub fn clear(&mut self) {
        self.virtual_buffer.clear();
    }
}
