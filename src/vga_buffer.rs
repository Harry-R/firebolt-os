use volatile::Volatile;
use core::fmt;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
/// struct to map color names to byte code for VGA buffer
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);
/// map 4bit foregrount color and 4bit background color into a single u8
impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
/// struct for a character, represented by ASCII code and color code
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}


#[repr(transparent)]
/// the volatile VGA shared memory buffer
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}


/// struct for VGA buffer writer
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}


/// implementation of VGA buffer writer
impl Writer {
    /// write a single byte to VGA buffer at correct position
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    /// write a newline to VGA buffer
    fn new_line(&mut self) {
        // TODO
    }

    /// write a string to VGA buffer, replace non-printable bytes
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // if ASCII printable or newline
                0x20...0x7e | b'\n' => self.write_byte(byte),
                // else
                _ => self.write_byte(0xfe),
            }
        }
    }
}


/// fmt::Write implementation for VGA buffer writer, so write! can be user
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}


/// test function for VGA buffer writer
pub fn print_something() {
    use core::fmt::Write;
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Green, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    write!(writer, "Hello world! {}", 42);
}