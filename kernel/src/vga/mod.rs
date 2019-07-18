//! # Video Graphics Array (VGA) Driver
//!
//! It allows to write in screen in ASCII
use core::{
    fmt::{self, Write},
    ptr,
};

mod character;

use crate::vga::character::Character;
pub use crate::vga::character::Color;

const ROWS: usize = 25;
const COLS: usize = 80;

/// VGA struct. It needs to receive a mutable slice of u8.
#[derive(Copy, Clone)]
pub struct Vga<T: AsMut<[u8]>> {
    slice: T,
    buffer: [Character; ROWS * COLS],
    position: usize,
    foreground: Color,
    background: Color,
}

impl<T: AsMut<[u8]>> Vga<T> {
    /// Creates a new instance of Vga.
    ///
    /// It have the default color as black for background
    /// and white to foreground.
    pub fn new(mut slice: T) -> Self {
        // We must have enough bytes of backing storage to make this work.
        assert_eq!(slice.as_mut().len(), ROWS * COLS * 2);

        // Default colors
        let foreground = Color::White;
        let background = Color::Black;

        let buffer = [Character::new(b' ', foreground, background); ROWS * COLS];

        Vga {
            slice,
            buffer,
            position: 0,
            foreground,
            background,
        }
    }

    /// Set a new foreground color
    pub fn set_foreground(&mut self, color: Color) {
        self.foreground = color;
    }

    /// Set a new background color
    pub fn set_background(&mut self, color: Color) {
        self.background = color;
    }

    /// Flush what it holds
    pub fn flush(&mut self) {
        // we need to use `write_volatile` here so that the writes aren't optimized out
        unsafe {
            let p = self.slice.as_mut();

            for (chunk, character) in p.chunks_mut(2).zip(self.buffer.iter()) {
                let (ch, attr) = character.as_bytes();

                let p = &mut chunk[0] as *mut u8;
                ptr::write_volatile(p, ch);

                let p = &mut chunk[1] as *mut u8;
                ptr::write_volatile(p, attr);
            }
        }
    }

    /// Scrolls a line
    fn scroll(&mut self) {
        for row in 1..ROWS {
            for cb in 0..COLS {
                let prev_pos = ((row - 1) * COLS) + cb;
                let curr_pos = (row * COLS) + cb;
                self.buffer[prev_pos] = self.buffer[curr_pos];
            }
        }

        for cb in 0..COLS {
            self.buffer[((ROWS - 1) * COLS) + cb] =
                Character::new(b' ', self.foreground, self.background);
        }

        self.position = (ROWS - 1) * COLS;
    }

    fn write_byte(&mut self, byte: u8) {
        let i = self.position;

        if byte == b'\n' {
            let curr_line = self.position / COLS;
            self.position = (curr_line + 1) * COLS;
        } else {
            self.buffer[i] = Character::new(byte, self.foreground, self.background);
            self.position += 1;
        }

        // Scrolls line if max buffer size have reached
        if self.position >= self.buffer.len() {
            self.scroll();
        }
    }
}

impl<T: AsMut<[u8]>> Write for Vga<T> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for b in s.bytes() {
            self.write_byte(b);
        }

        Ok(())
    }
}
