fn sq_start(is_bg: bool) -> &'static str {
	if is_bg { "48" } else { "38" }
}

pub enum PaletteColor {
	Black,
	Red,
	Green,
	Yellow,
	Blue,
	Magenta,
	Cyan,
	White,

	BrightBlack,
	BrightRed,
	BrightGreen,
	BrightYellow,
	BrightBlue,
	BrightMagenta,
	BrightCyan,
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

pub enum Color {
	Palette(PaletteColor),
	Rgb8bit(u8),
	Rgb24bit(u8, u8, u8),
}
impl Color {
	pub fn gen_sequence(&self, background: bool) -> String {
		match self {
			Color::Palette(palette) => {
				let mut code = palette.get_code();

				if background {
					code += 10;
				}

				code.to_string()
			}
			Color::Rgb8bit(code) => {
				format!("{};5;{}", sq_start(background), code)
			}
			Color::Rgb24bit(r, g, b) => {
				format!("{};2;{};{};{}", sq_start(background), r, g, b)
			}
		}
	}
}
