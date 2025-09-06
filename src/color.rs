fn sq_start(is_bg: bool) -> &'static str {
	if is_bg { "48" } else { "38" }
}

/// Used to specify the color in `Color::Palette`
///
/// <https://en.wikipedia.org/wiki/ANSI_escape_code#3-bit_and_4-bit>
pub enum PaletteColor {
	/// Color #0, may display as the terminal's default background color.
	Black,
	/// Color #1
	Red,
	/// Color #2
	Green,
	/// Color #3
	Yellow,
	/// Color #4
	Blue,
	/// Color #5
	Magenta,
	/// Color #6
	Cyan,
	/// Color #7
	White,

	/// Color #8, Gray
	BrightBlack,
	/// Color #9
	BrightRed,
	/// Color #10
	BrightGreen,
	/// Color #11
	BrightYellow,
	/// Color #12
	BrightBlue,
	/// Color #13
	BrightMagenta,
	/// Color #14
	BrightCyan,
	/// Color #15
	BrightWhite,
}
impl PaletteColor {
	fn get_code(&self) -> u8 {
		match self {
			PaletteColor::Black => 30,
			PaletteColor::Red => 31,
			PaletteColor::Green => 32,
			PaletteColor::Yellow => 33,
			PaletteColor::Blue => 34,
			PaletteColor::Magenta => 35,
			PaletteColor::Cyan => 36,
			PaletteColor::White => 37,

			PaletteColor::BrightBlack => 90,
			PaletteColor::BrightRed => 91,
			PaletteColor::BrightGreen => 92,
			PaletteColor::BrightYellow => 93,
			PaletteColor::BrightBlue => 94,
			PaletteColor::BrightMagenta => 95,
			PaletteColor::BrightCyan => 96,
			PaletteColor::BrightWhite => 97,
		}
	}
}

/// Used for specifying the color to set the background or foreground of an `AnsiStyle`
pub enum Color {
	/// One of the predefined colors in the `PaletteColor` enum.
	///
	/// These are identical to the colors from `Fixed8bit(0-15)`, but instead are formatted as `XX` instead of `#8;5;XX` (color may vary depending on terminal!)
	Palette(PaletteColor),

	/// A color from a predefined 256 color table
	///
	/// - 0-7: standard palette colors `Black` through `White`
	/// - 8-15: brighter palette colors `BrightBlack` through `BrightWhite`
	/// - 16-231: colors inside a 6x6x6 cube `16 + (36 * red) + (6 * green) + blue`
	/// - 232-255: grayscale from black to white in 24 steps
	Fixed8bit(u8),

	/// A 24-bit RGB color
	Rgb24bit(u8, u8, u8),
}
impl Color {
	pub(crate) fn gen_sequence(&self, background: bool) -> String {
		match self {
			Color::Palette(palette) => {
				let mut code = palette.get_code();

				if background {
					code += 10;
				}

				code.to_string()
			}
			Color::Fixed8bit(code) => {
				format!("{};5;{}", sq_start(background), code)
			}
			Color::Rgb24bit(r, g, b) => {
				format!("{};2;{};{};{}", sq_start(background), r, g, b)
			}
		}
	}
}
