mod bit_masks {
	pub const BOLD: u8 = 0b1;
	pub const ITALIC: u8 = 0b10;
	pub const UNDERLINE: u8 = 0b100;
	pub const STRIKETHROUGH: u8 = 0b1000;
}

use std::fmt::{Display, Write};

use super::{Color, codes};

#[derive(Default)]
pub struct AnsiStyle {
	pub foreground: Option<Color>,
	pub background: Option<Color>,
	pub flags: u8,
}
impl AnsiStyle {
	pub fn new() -> Self {
		Self {
			..Default::default()
		}
	}

	// getters
	pub fn g_bold(&self) -> bool {
		self.flags & bit_masks::BOLD == bit_masks::BOLD
	}
	pub fn g_italic(&self) -> bool {
		self.flags & bit_masks::ITALIC == bit_masks::ITALIC
	}
	pub fn g_underline(&self) -> bool {
		self.flags & bit_masks::UNDERLINE == bit_masks::UNDERLINE
	}
	pub fn g_strikethrough(&self) -> bool {
		self.flags & bit_masks::STRIKETHROUGH == bit_masks::STRIKETHROUGH
	}

	fn set_flags(&mut self, mask: u8, enabled: bool) -> &mut Self {
		if enabled {
			self.flags |= mask;
		} else {
			self.flags &= !mask;
		}
		self
	}
	fn b_set_flags(mut self, mask: u8, enabled: bool) -> Self {
		self.set_flags(mask, enabled);
		self
	}

	pub fn is_plain(&self) -> bool {
		!((self.flags != 0) | self.foreground.is_some() | self.background.is_some())
	}

	// mutating chaining
	pub fn m_bold(&mut self, enabled: bool) -> &mut Self {
		self.set_flags(bit_masks::BOLD, enabled)
	}
	pub fn m_italic(&mut self, enabled: bool) -> &mut Self {
		self.set_flags(bit_masks::ITALIC, enabled)
	}
	pub fn m_underline(&mut self, enabled: bool) -> &mut Self {
		self.set_flags(bit_masks::UNDERLINE, enabled)
	}
	pub fn m_strikethrough(&mut self, enabled: bool) -> &mut Self {
		self.set_flags(bit_masks::STRIKETHROUGH, enabled)
	}
	pub fn m_fg(&mut self, foreground: Option<Color>) -> &mut Self {
		self.foreground = foreground;
		self
	}
	pub fn m_bg(&mut self, background: Option<Color>) -> &mut Self {
		self.background = background;
		self
	}

	// borrow mutating chaining
	pub fn bm_bold(self, enabled: bool) -> Self {
		self.b_set_flags(bit_masks::BOLD, enabled)
	}
	pub fn bm_italic(self, enabled: bool) -> Self {
		self.b_set_flags(bit_masks::ITALIC, enabled)
	}
	pub fn bm_underline(self, enabled: bool) -> Self {
		self.b_set_flags(bit_masks::UNDERLINE, enabled)
	}
	pub fn bm_strikethrough(self, enabled: bool) -> Self {
		self.b_set_flags(bit_masks::STRIKETHROUGH, enabled)
	}
	pub fn bm_fg(mut self, foreground: Option<Color>) -> Self {
		self.foreground = foreground;
		self
	}
	pub fn bm_bg(mut self, background: Option<Color>) -> Self {
		self.background = background;
		self
	}

	pub fn m_apply(&self, text: &mut String) {
		text.insert_str(0, &self.to_string()[..]);
	}
	pub fn m_apply_with_reset(&self, text: &mut String) {
		if self.is_plain() {
			return;
		}
		self.m_apply(text);
		text.push_str(codes::RESET);
	}

	// constructing chaining
	pub fn bold(&self) -> Self {
		Self {
			flags: self.flags | bit_masks::BOLD,
			..Default::default()
		}
	}
	pub fn italic(&self) -> Self {
		Self {
			flags: self.flags | bit_masks::ITALIC,
			..Default::default()
		}
	}
	pub fn underline(&self) -> Self {
		Self {
			flags: self.flags | bit_masks::UNDERLINE,
			..Default::default()
		}
	}
	pub fn strikethrough(&self) -> Self {
		Self {
			flags: self.flags | bit_masks::STRIKETHROUGH,
			..Default::default()
		}
	}
	pub fn fg(&self, foreground: Color) -> Self {
		Self {
			foreground: Some(foreground),
			..Default::default()
		}
	}
	pub fn bg(&self, background: Color) -> Self {
		Self {
			background: Some(background),
			..Default::default()
		}
	}

	pub fn apply(&self, text: &str) -> String {
		let mut new_text = text.to_owned();
		self.m_apply(&mut new_text);
		new_text
	}
	pub fn apply_with_reset(&self, text: &str) -> String {
		let mut new_text = text.to_owned();
		self.m_apply_with_reset(&mut new_text);
		new_text
	}
}
impl Display for AnsiStyle {
	fn fmt(&self, handle: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.is_plain() {
			return Ok(()); // i wish there was a way to propogate that the formatting *technically* failed because the style is plain, but we can't :/ (or atleast i don't know how to yet)
		}

		handle.write_str("\x1b[")?;

		let mut wrote = false;
		let mut append = |arg: &str| -> std::fmt::Result {
			if wrote {
				handle.write_char(';')?;
			}
			handle.write_str(arg)?;
			wrote = true;
			Ok(())
		};

		if self.g_bold() {
			append("1")?
		}
		if self.g_italic() {
			append("3")?
		}
		if self.g_underline() {
			append("4")?
		}
		if self.g_strikethrough() {
			append("9")?
		}

		if let Some(foreground) = &self.foreground {
			append(foreground.gen_sequence(false).as_str())?
		}
		if let Some(background) = &self.background {
			append(background.gen_sequence(true).as_str())?
		}

		handle.write_char('m')?;

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new_style_is_plain() {
		let style = AnsiStyle::new();

		assert!(style.is_plain(), "Not plain")
	}

	#[test]
	fn flag_positions() {
		let style = AnsiStyle {
			flags: 0b101,
			..Default::default()
		};

		assert!(style.g_bold(), "Style should be bold");
		assert!(!style.g_italic(), "Style shouldn't be italic");
		assert!(style.g_underline(), "Style should be underlined");
		assert!(!style.g_strikethrough(), "Style shouldn't be strikethrough");
	}
	#[test]
	fn mutate_flags() {
		let mut style = AnsiStyle::new();
		assert_eq!(style.flags, 0, "flags should be 0");

		style.m_bold(true);
		assert_eq!(style.flags, 1, "flags should be 1");

		style.m_italic(true);
		assert_eq!(style.flags, 3, "flags should be 3");

		style.m_underline(true);
		assert_eq!(style.flags, 7, "flags should be 7");

		style.m_strikethrough(true);
		assert_eq!(style.flags, 15, "flags should be 15");
	}

	#[test]
	fn plain_m_apply_shouldnt_modify() {
		let plain_style = AnsiStyle::new();
		let mut text = String::from("Hello world!");

		plain_style.m_apply(&mut text);

		assert_eq!(
			&text[..],
			"Hello world!",
			"String was modified by a plain style, got {text}"
		);

		plain_style.m_apply_with_reset(&mut text);

		assert_eq!(
			&text[..],
			"Hello world!",
			"String was modified by a plain style, got {text}"
		)
	}
	#[test]
	fn m_apply_modify() {
		let style = AnsiStyle {
			flags: 0b1,
			..Default::default()
		};
		let mut text = String::from("Hello world!");

		assert_eq!(
			&text[..],
			"Hello world!" // "String doesn't equal Hello world?"
		);

		style.m_apply(&mut text);

		assert_eq!(
			&text[..],
			"\x1b[1mHello world!",
			"Expected: {:3?}\nGot: {:3?}",
			"\x1b[1mHello world!".as_bytes(),
			text.as_bytes()
		);
	}

	#[test]
	fn plain_apply_shouldnt_differ() {
		let style = AnsiStyle::new();
		let input_text = String::from("Hello world!");

		assert_eq!(
			&input_text[..],
			"Hello world!" // "String doesn't equal Hello world?"
		);

		let output_text = style.apply(&input_text);

		assert_eq!(
			output_text,
			input_text //"Output text doesn't match input text"
		)
	}
}
