//! A definition how it is structured a character in VGA driver

/// Colors allowed in ASCII
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    Gray = 0x7,
    DarkGray = 0x8,
    BrightBlue = 0x9,
    BrightGreen = 0xA,
    BrightCyan = 0xB,
    BrightRed = 0xC,
    BrightMagenta = 0xD,
    Yellow = 0xE,
    White = 0xF,
}

/// In VGA a character is represented by a 16 bits,
/// the first 8 represent the character in ASCII code
/// and the last 8 represents the atribute for the character.
///
/// In the 8 bits of the attribute, the first 4 bits represents
/// the background color and the last 4 the foreground color.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Character {
    character: u8,
    attribute: u8,
}

impl Character {
    /// Creates a new instance of Character.
    pub fn new(character: u8, foreground: Color, background: Color) -> Self {
        let attribute = ((background as u8) << 4) + (foreground as u8);

        Character {
            character,
            attribute
        }
    }

    /// Converts Character into a tuple of bytes
    pub fn as_bytes(&self) -> (u8, u8) {
        (self.character, self.attribute)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation() {
        let character = Character::new(b'a', Color::Blue, Color::BrightMagenta);

        assert_eq!(character.character, b'a');
        assert_eq!(character.attribute, 0xD1);

        let character = Character::new(b'b', Color::Yellow, Color::Red);

        assert_eq!(character.character, b'b');
        assert_eq!(character.attribute, 0x4E);

        let character = Character::new(b'c', Color::DarkGray, Color::White);

        assert_eq!(character.character, b'c');
        assert_eq!(character.attribute, 0xF8);
    }
}
