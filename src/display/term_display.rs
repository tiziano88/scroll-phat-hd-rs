use super::*;

/// A virtual display that outputs its buffer to the terminal from which the binary is attached.
///
/// Useful for debugging or prototyping, as it does not require a physical display to be connected.
pub struct TermDisplay {}

impl TermDisplay {
    pub fn new() -> TermDisplay {
        TermDisplay {}
    }
}

impl Display for TermDisplay {
    fn show(&mut self, buffer: &[Column]) -> Result<(), Error> {
        print!("{}", termion::clear::All);
        for x in 0..buffer.len() {
            let col = &buffer[x];
            for y in 0..col.len() {
                let c = col[y];
                let v = if c == 0 { ' ' } else { '#' };
                println!("{}{}", termion::cursor::Goto(x as u16 + 1, y as u16 + 1), v);
            }
        }
        Ok(())
    }
}
