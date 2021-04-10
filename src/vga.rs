use volatile::Volatile;
use core::fmt;

const VGA_SIZE_H: usize = 25;
const VGA_SIZE_W: usize = 80;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct Character {
    ascii: u8,
    color: ColorCode,
}

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<Character>; VGA_SIZE_W]; VGA_SIZE_H],
}

pub struct Writer {
    col_pos: usize,
    color: ColorCode,
    buffer: &'static mut Buffer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
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
    pub const BLANK: ColorCode = ColorCode((Color::Black as u8) << 4 | (Color::Black as u8));

    fn new(text: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (text as u8))
    }
}

impl Writer {
    const BLANK: Character = Character {
        ascii: b' ',
        color: ColorCode::BLANK,
    };

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

                self.buffer.chars[row][col].write(Character {
                    ascii: byte,
                    color,
                });

                self.col_pos += 1;
            }
        }
    }

    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe)
            }
        }
    }

    pub fn shift_rows(&mut self, distance: i32) {
        for row in 1..VGA_SIZE_H {
            if (row as i32 + distance) > 0 {
                let unsigned: usize = row + distance as usize;
                for col in 0..VGA_SIZE_W {
                    let char = self.buffer.chars[row][col].read();
                    self.buffer.chars[unsigned][col].write(char);
                }
            }
        }
    }

    pub fn clear_row(&mut self, row: usize) {
        for col in 0..VGA_SIZE_W {
            self.buffer.chars[row][col].write(Writer::BLANK);
        }
    }

    pub fn clear_col(&mut self, col: usize) {
        for row in 0..VGA_SIZE_H {
            self.buffer.chars[row][col].write(Writer::BLANK);
        }
    }

    pub fn new_line(&mut self) {
        self.shift_rows(-1);
        self.clear_row(VGA_SIZE_H - 1);
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        self.write_string(string);
        Ok(())
    }
}

pub fn write(string: &str, color: Color, background: Color) {
    let mut writer = Writer {
        col_pos: 0,
        color: ColorCode::new(color, background),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_string(string);
}

pub fn print_string() {
    let colors: [ColorCode; 5] = [
        ColorCode::new(Color::White, Color::Black),
        ColorCode::new(Color::Pink, Color::Black),
        ColorCode::new(Color::BlueDark, Color::BlueLight),
        ColorCode::new(Color::GreenLight, Color::GreenDark),
        ColorCode::new(Color::GreenDark, Color::GreenLight)
    ];

    write("hello noelia v2 ", Color::GreenLight, Color::GreenDark);
}
