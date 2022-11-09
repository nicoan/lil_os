///! Generic definitions for text mode screens
pub mod vga;

pub const DEFAULT_FOREGROUND: PrintColor = PrintColor::LightGray;
pub const DEFAULT_BACKGROUND: PrintColor = PrintColor::Black;

/// Text colors
#[allow(dead_code)]
pub enum PrintColor {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    Pink,
    Yellow,
    White,
}

pub(crate) trait Writer {
    fn write_byte(&mut self, byte: u8);

    fn write_string(&mut self, s: &str);

    fn clear_row(&mut self, row: usize);

    fn new_line(&mut self);

    fn set_color(&mut self, foreground: PrintColor, background: PrintColor);

    fn clear_screen(&mut self, background: Option<PrintColor>);
}

#[macro_export]
macro_rules! print {
    ([$foreground: expr, $background: expr], $($arg:tt)*) => {
        $crate::drivers::screen::text::vga::_set_color($foreground, $background);
        $crate::print!("{}", format_args!($($arg)*));
        $crate::drivers::screen::text::vga::_set_color(
            $crate::drivers::screen::text::DEFAULT_FOREGROUND,
            $crate::drivers::screen::text::DEFAULT_BACKGROUND
        );
    };
    ([$foreground: expr], $($arg:tt)*) => {
        $crate::drivers::screen::text::vga::_set_color($foreground, $crate::drivers::screen::text::DEFAULT_BACKGROUND);
        $crate::print!("{}", format_args!($($arg)*));
        $crate::drivers::screen::text::vga::_set_color(
            $crate::drivers::screen::text::DEFAULT_FOREGROUND,
            $crate::drivers::screen::text::DEFAULT_BACKGROUND
        );
    };
    ($($arg:tt)*) => ($crate::drivers::screen::text::vga::_print(format_args!($($arg)*)));
}

pub use print;

#[macro_export]
macro_rules! println {
    ([$foreground: expr, $background: expr], $($arg:tt)*) => {
        $crate::print!([$foreground, $background], "{}\n", format_args!($($arg)*));
    };
    ([$foreground: expr], $($arg:tt)*) => {
        $crate::print!([$foreground], "{}\n", format_args!($($arg)*));
    };
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! panic_screen {
    ($($arg:tt)*) => {
        $crate::drivers::screen::text::vga::_set_color(
            $crate::drivers::screen::text::PrintColor::LightGray,
            $crate::drivers::screen::text::PrintColor::Blue
        );
        $crate::drivers::screen::text::vga::_clear_screen(
            Some($crate::drivers::screen::text::PrintColor::Blue)
        );
        $crate::print!("{}\n", format_args!($($arg)*));
    }
}

pub use println;
