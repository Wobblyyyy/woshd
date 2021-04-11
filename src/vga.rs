use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

/// Horizontal size of the VGA buffer (ROWS)
const VGA_SIZE_H: usize = 25;

/// Vertical size of the VGA buffer (COLS)
const VGA_SIZE_W: usize = 80;

/// VGA memory address - VGA is a statically mapped memory buffer.
const VGA_MEM: usize = 0xb8000;

/// A single color code, comprised of... well, a color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

/// A single character that can be written to the VGA buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct Character {
    ascii: u8,
    color: ColorCode,
}

/// A VGA buffer instance.
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<Character>; VGA_SIZE_W]; VGA_SIZE_H],
}

/// The screen writer - print characters, strings, etc to the screen.
///
/// This struct makes use of the VGA buffer to display text as you'd expect.
pub struct Writer {
    col_pos: usize,
    color: ColorCode,
    buffer: &'static mut Buffer,
}

/// Enumeration of colors.
///
/// The names of the colors are as follows:
/// {Color}{Modifier}
/// Where color is the "base" color and modifier is the differentiating factor of the color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    BlueDark = 1,
    GreenDark = 2,
    CyanDark = 3,
    RedDark = 4,
    Magenta = 5,
    Brown = 6,
    GrayLight = 7,
    GrayDark = 8,
    BlueLight = 9,
    GreenLight = 10,
    CyanLight = 11,
    RedLight = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

impl ColorCode {
    /// Blank color code - black background, black foreground.
    pub const BLANK: ColorCode = ColorCode((Color::Black as u8) << 4 | (Color::Black as u8));

    /// Create a new ColorCode
    ///
    /// Text: FOREGROUND TEXT COLOR
    /// Background: BACKGROUND COLOR
    fn new(text: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (text as u8))
    }
}

impl Character {
    /// Blank character - SPACE as ASCII value, black background, black foreground.
    const BLANK: Character = Character {
        ascii: b' ',
        color: ColorCode::BLANK,
    };
}

impl Buffer {
    /// Write a single character to the VGA buffer.
    ///
    /// # Parameters
    /// char: the character to write to the VGA buffer.
    /// row: the row that the character should go in.
    /// col: the col that the character should go in.
    pub fn write_char(&mut self, char: Character, row: usize, col: usize) {
        if row < VGA_SIZE_H && col < VGA_SIZE_W {
            self.chars[row][col].write(char);
        }
    }

    /// Write a byte and color combination.
    ///
    /// # Parameters
    /// byte: the byte data to write.
    /// color: the color combination for the byte.
    /// row: the row the byte should go into.
    /// col: the col the byte should go into.
    pub fn write_byte_with_color(
        &mut self,
        byte: u8,
        color: ColorCode,
        row: usize,
        col: usize,
    ) {
        let as_char = Character {
            ascii: byte,
            color,
        };

        self.write_char(as_char, row, col);
    }
}

impl Writer {
    /// Blank character that an be written. Migrate away from using this one?
    const BLANK: Character = Character {
        ascii: b' ',
        color: ColorCode::BLANK,
    };

    /// Write a single byte to the buffer. This uses the "col_pos" of the Writer instance you're
    /// using. Use the buffer's set method to actually by position. If the byte that's written
    /// is a newline (b'\n'), call the new_line method.
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col_pos >= VGA_SIZE_W {
                    self.new_line();
                }

                let row = VGA_SIZE_H - 1;
                let col = self.col_pos;
                let color = self.color;

                self.buffer.write_byte_with_color(
                    byte,
                    color,
                    row,
                    col,
                );

                self.col_pos += 1;
            }
        }
    }

    /// Write a string to the terminal. Calls the write_byte method repeatedly. If the string is
    /// longer than the amount of available space, automatically insert a newline in the middle
    /// of the screen.
    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe)
            }
        }
    }

    /// Shift the buffer's contents upwards by a given distance.
    /// CLIPPING ENABLED: some characters will get clipped.
    pub fn shift_up(&mut self, distance: usize) {
        for row in 1..VGA_SIZE_H {
            for col in 0..VGA_SIZE_W {
                let char = self.buffer.chars[row][col].read();
                self.buffer.chars[row - distance][col].write(char);
            }

            if row >= VGA_SIZE_H - distance {
                self.clear_row(row)
            }
        }
    }

    /// Shift the buffer's contents downwards by a given distance.
    /// CLIPPING ENABLED: some characters will get clipped.
    pub fn shift_down(&mut self, distance: usize) {
        for row in 0..VGA_SIZE_H {
            for col in 0..VGA_SIZE_W {
                let char = self.buffer.chars[row][col].read();
                self.buffer.chars[row + distance][col].write(char);
            }

            if row < distance {
                self.clear_row(row)
            }
        }
    }

    /// Shift the buffer's contents to the right.
    /// CLIPPING ENABLED: some characters will get clipped.
    pub fn shift_right(&mut self, distance: usize) {
        for col in 0..VGA_SIZE_W {
            for row in 0..VGA_SIZE_H {
                let char = self.buffer.chars[row][col].read();
                self.buffer.chars[row][col + distance].write(char);
            }

            if col < distance {
                self.clear_col(col);
            }
        }
    }

    /// Shift the buffer's contents to the left.
    /// CLIPPING ENABLED: some characters will get clipped.
    pub fn shift_left(&mut self, distance: usize) {
        for col in 0..VGA_SIZE_W {
            for row in 0..VGA_SIZE_H {
                let char = self.buffer.chars[row][col].read();
                self.buffer.chars[row][col + distance].write(char);
            }

            if col > VGA_SIZE_W - distance {
                self.clear_col(col);
            }
        }
    }

    /// Clear a specified row entirely.
    /// This sets the contents of the row to the BLANK character.
    pub fn clear_row(&mut self, row: usize) {
        for col in 0..VGA_SIZE_W {
            self.buffer.chars[row][col].write(Writer::BLANK);
        }
    }

    /// Clear an entire column.
    /// This sets the contents of the row to the BLANK character.
    pub fn clear_col(&mut self, col: usize) {
        for row in 0..VGA_SIZE_H {
            self.buffer.chars[row][col].write(Writer::BLANK);
        }
    }

    /// Shift the terminal's contents upwards by 1 and reset the column position to 0.
    pub fn new_line(&mut self) {
        self.shift_up(1);
        self.col_pos = 0;
        // self.clear_row(VGA_SIZE_H - 1);
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        self.write_string(string);
        Ok(())
    }
}

// pub fn write(string: &str, color: Color, background: Color) {
//     let mut writer = Writer {
//         col_pos: 0,
//         color: ColorCode::new(color, background),
//         buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
//     };
//
//     writer.write_string(string);
// }

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        col_pos: 0,
        color: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::vga::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
