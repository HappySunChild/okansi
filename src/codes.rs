pub const BELL: &str = "\x07";
pub const RESET: &str = "\x1b[0m";
pub const HOME: &str = "\x1b[H";

pub mod erase {
	pub const SCREEN: &str = "\x1b[2J";
	pub const CURSOR_TO_END_SCREEN: &str = "\x1b[0J";
	pub const CURSOR_TO_START_SCREEN: &str = "\x1b[1J";

	pub const LINE: &str = "\x1b[2K";
	pub const CURSOR_TO_END_LN: &str = "\x1b[0K";
	pub const CURSOR_TO_START_LN: &str = "\x1b[0K";
}
