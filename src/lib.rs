#[cfg(target_os = "windows")]
extern crate winapi;

pub mod codes;

mod color;
pub use color::{Color, PaletteColor};

mod style;
pub use style::AnsiStyle;

pub mod windows;
