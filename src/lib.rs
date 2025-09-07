#[cfg(target_os = "windows")]
extern crate winapi;

/// Windows related functionality
#[cfg(windows)]
pub mod windows;

/// ANSI escape code constants
pub mod codes;

mod color;
pub use color::{Color, PaletteColor};

mod style;
pub use style::AnsiStyle;
