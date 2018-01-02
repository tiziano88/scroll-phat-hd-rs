use super::*;

/// A virtual display that outputs its buffer to the terminal from which the binary is attached.
///
/// Useful for debugging or prototyping, as it does not require a physical display to be connected.
pub struct UnicodeDisplay {}

impl UnicodeDisplay {
    pub fn new() -> UnicodeDisplay {
        UnicodeDisplay {}
    }
}

impl Display for UnicodeDisplay {
    fn show(&mut self, buffer: &[Column]) -> Result<(), Error> {
        print!("{}", termion::clear::All);

        let col = &buffer[0];

        println!("{}╔", termion::cursor::Goto(1, 1));
        for y in 0..LED_ROWS {
            println!("{}║", termion::cursor::Goto(1, y as u16 + 2));
        }
        println!("{}╚", termion::cursor::Goto(1, LED_ROWS as u16 + 2));

        for x in 0..LED_COLUMNS {
            println!("{}═", termion::cursor::Goto(x as u16 + 2, 1));
            if let Some(ref col) = buffer.get(x) {
                for y in 0..LED_ROWS {
                    let c = col[y];
                    let v = if c == 0 { '░' } else { '▓' };
                    println!("{}{}", termion::cursor::Goto(x as u16 + 2, y as u16 + 2), v);
                }
            } else {
                for y in 0..LED_ROWS {
                    let v = '░';
                    println!("{}{}", termion::cursor::Goto(x as u16 + 2, y as u16 + 2), v);
                }
            }
            println!(
                "{}═",
                termion::cursor::Goto(x as u16 + 2, LED_ROWS as u16 + 2)
            );
        }

        println!("{}╗", termion::cursor::Goto(LED_COLUMNS as u16 + 1, 1));
        for y in 0..LED_ROWS {
            println!(
                "{}║",
                termion::cursor::Goto(LED_COLUMNS as u16 + 1, y as u16 + 2)
            );
        }
        println!(
            "{}╝",
            termion::cursor::Goto(LED_COLUMNS as u16 + 1, col.len() as u16 + 2)
        );

        Ok(())
    }
}
