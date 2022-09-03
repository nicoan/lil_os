//! VGA Text mode buffer driver
//! Most of the code is from https://os.phil-opp.com/vga-text-mode/
use crate::drivers::screen::text::Writer;
use core::clone::Clone;
use core::fmt::Write;
use core::marker::Copy;
use core::prelude::v1::derive;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

// The vga buffer is a 80x25 matrix
const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;
const BUFFER_LAST_ROW: usize = BUFFER_HEIGHT - 1;
const DEFAULT_COLOR: ColorCode = ColorCode::new(Color::LightGray, Color::Black);

lazy_static! {
    pub static ref WRITER: Mutex<VGAWriter> = Mutex::new(VGAWriter {
        column_position: 0,
        color: DEFAULT_COLOR,
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

// We only need 4 bits to represent the color, but Rust does not have a u4 type, so we use the
// minimum unsigned integer possible: u8.
// The repr(u8) macro indicates that every variant should be stored as a u8
#[allow(dead_code)]
#[repr(u8)]
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xa,
    LightCyan = 0xb,
    LightRed = 0xc,
    Pink = 0xd,
    Yellow = 0xe,
    White = 0xf,
}

impl From<super::PrintColor> for Color {
    fn from(color: super::PrintColor) -> Self {
        match color {
            super::PrintColor::Black => Self::Black,
            super::PrintColor::Blue => Self::Blue,
            super::PrintColor::Green => Self::Green,
            super::PrintColor::Cyan => Self::Cyan,
            super::PrintColor::Red => Self::Red,
            super::PrintColor::Magenta => Self::Magenta,
            super::PrintColor::Brown => Self::Brown,
            super::PrintColor::LightGray => Self::LightGray,
            super::PrintColor::DarkGray => Self::DarkGray,
            super::PrintColor::LightBlue => Self::LightBlue,
            super::PrintColor::LightGreen => Self::LightGreen,
            super::PrintColor::LightCyan => Self::LightCyan,
            super::PrintColor::LightRed => Self::LightRed,
            super::PrintColor::Pink => Self::Pink,
            super::PrintColor::Yellow => Self::Yellow,
            super::PrintColor::White => Self::White,
        }
    }
}

/// Represents a full color byte. The first 4 bytes are the background color and the later 4 are
/// the foreground.
/// repr(transparent) allow us to treat the structure as it was a simple u8.
#[repr(transparent)]
#[derive(Copy, Clone)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

/// Represents a character in the VGA buffer. First 8 bits are the character itsef and the later 8
/// bits are the color.
// The repr(C) macro guarantees that the struct's fields are laid out exactly like a C struct, and
// thus guarantees the correct field ordering.
#[repr(C)]
#[derive(Copy, Clone)]
struct ScreenCharacter {
    /// Ascii char code
    character: u8,

    /// Color code
    color: ColorCode,
}

/// Represents the caracter buffer. This buffer will be mapped to the actual VGA text buffer
/// located at the address 0xb8000.
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenCharacter>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct VGAWriter {
    /// The cursor's current position
    column_position: usize,

    /// Current color (foreground and background)
    color: ColorCode,

    /// Reference to the VGA buffer
    buffer: &'static mut Buffer,
}

impl Writer for VGAWriter {
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                // If we are in the limit of the buffer we make a new line
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                // Otherwise we print the character
                self.buffer.chars[BUFFER_LAST_ROW][self.column_position].write(ScreenCharacter {
                    character: byte,
                    color: self.color,
                });

                // We go to the next position
                self.column_position += 1;
            }
        }
    }

    /// Writes a string into the VGA text buffer.
    ///
    /// The VGA text buffer only supports ASCII and the additional bytes of code page 437. Rust
    /// strings are UTF-8 by default, so they might contain bytes that are not supported by the VGA
    /// text buffer. We use a match to differentiate printable ASCII bytes (a newline or anything
    /// in between a space character and a ~ character) and unprintable bytes. For unprintable
    /// bytes, we print a ■ character, which has the hex code 0xfe on the VGA hardware.
    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // Is a printable ASCII byte or a new line.
                0x20..=0x7e | b'\n' => self.write_byte(byte),

                // Not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    /// Clears the row with the default color
    fn clear_row(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(ScreenCharacter {
                character: b' ',
                color: DEFAULT_COLOR,
            });
        }
    }

    /// Moves all the characters one row up and clears the last row
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_LAST_ROW);
        self.column_position = 0;
    }

    /// Sets the text background and foreground color
    fn set_color(&mut self, foreground: super::PrintColor, background: super::PrintColor) {
        self.color = ColorCode::new(foreground.into(), background.into());
    }
}

/// This allow us to use Rust's formatting macros!
impl core::fmt::Write for VGAWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}

#[doc(hidden)]
pub fn _set_color(foreground: super::PrintColor, background: super::PrintColor) {
    WRITER.lock().set_color(foreground, background);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn test_println_many() {
        for _ in 0..200 {
            crate::println!("test_print output");
        }
    }

    #[test_case]
    fn test_println_output() {
        let s = "Some test string that fits on a single line";
        crate::println!("{}", s);
        for (i, c) in s.chars().enumerate() {
            let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.character), c);
        }
    }
}
