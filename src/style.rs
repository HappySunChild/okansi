mod bit_masks {
	pub const BOLD: u8 = 0b1;
	pub const ITALIC: u8 = 0b10;
	pub const UNDERLINE: u8 = 0b100;
	pub const STRIKETHROUGH: u8 = 0b1000;
	pub const DIM: u8 = 0b10000;
	pub const BLINKING: u8 = 0b100000;
	pub const REVERSE: u8 = 0b1000000;
	pub const HIDDEN: u8 = 0b10000000;
}

use std::fmt::{Display, Write};

use super::{Color, codes};

/// Contains data and methods for easily configuring ANSI format sequences
#[derive(Default)]
pub struct AnsiStyle {
	pub foreground: Option<Color>,
	pub background: Option<Color>,
	pub flags: u8,
}
impl AnsiStyle {
	/// Returns a new plain `AnsiStyle`
	pub fn new() -> Self {
		Self {
			..Default::default()
		}
	}

	/// Returns whether the bold flag is set
	pub fn g_bold(&self) -> bool {
		self.flags & bit_masks::BOLD == bit_masks::BOLD
	}
	/// Returns whether the italic flag is set
	pub fn g_italic(&self) -> bool {
		self.flags & bit_masks::ITALIC == bit_masks::ITALIC
	}
	/// Returns whether the underline flag is set
	pub fn g_underline(&self) -> bool {
		self.flags & bit_masks::UNDERLINE == bit_masks::UNDERLINE
	}
	/// Returns whether the strikethough flag is set
	pub fn g_strikethrough(&self) -> bool {
		self.flags & bit_masks::STRIKETHROUGH == bit_masks::STRIKETHROUGH
	}
	/// Returns whether the dim flag is set
	pub fn g_dim(&self) -> bool {
		self.flags & bit_masks::DIM == bit_masks::DIM
	}
	/// Returns whether the blinking flag is set
	pub fn g_blinking(&self) -> bool {
		self.flags & bit_masks::BLINKING == bit_masks::BLINKING
	}
	/// Returns whether the reverse flag is set
	pub fn g_reverse(&self) -> bool {
		self.flags & bit_masks::REVERSE == bit_masks::REVERSE
	}
	/// Returns whether the hidden flag is set
	pub fn g_hidden(&self) -> bool {
		self.flags & bit_masks::HIDDEN == bit_masks::HIDDEN
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

	/// Returns whether the `AnsiStyle` will be applied
	pub fn is_plain(&self) -> bool {
		!((self.flags != 0) | self.foreground.is_some() | self.background.is_some())
	}

	/// Modifies the `AnsiStyle`'s bold flag, must be mutable
	pub fn m_bold(&mut self, enabled: bool) -> &mut Self {
		self.set_flags(bit_masks::BOLD, enabled)
	}
	/// Modifies the `AnsiStyle`'s italic flag, must be mutable
	pub fn m_italic(&mut self, enabled: bool) -> &mut Self {
		self.set_flags(bit_masks::ITALIC, enabled)
	}
	/// Modifies the `AnsiStyle`'s underline flag, must be mutable
	pub fn m_underline(&mut self, enabled: bool) -> &mut Self {
		self.set_flags(bit_masks::UNDERLINE, enabled)
	}
	/// Modifies the `AnsiStyle`'s strikethrough flag, must be mutable
	pub fn m_strikethrough(&mut self, enabled: bool) -> &mut Self {
		self.set_flags(bit_masks::STRIKETHROUGH, enabled)
	}
	/// Modifies the `AnsiStyle`'s dim flag, must be mutable
	pub fn m_dim(&mut self, enabled: bool) -> &mut Self {
		self.set_flags(bit_masks::DIM, enabled)
	}
	/// Modifies the `AnsiStyle`'s blinking flag, must be mutable
	pub fn m_blinking(&mut self, enabled: bool) -> &mut Self {
		self.set_flags(bit_masks::BLINKING, enabled)
	}
	/// Modifies the `AnsiStyle`'s reverse flag, must be mutable
	pub fn m_reverse(&mut self, enabled: bool) -> &mut Self {
		self.set_flags(bit_masks::REVERSE, enabled)
	}
	/// Modifies the `AnsiStyle`'s hidden flag, must be mutable
	pub fn m_hidden(&mut self, enabled: bool) -> &mut Self {
		self.set_flags(bit_masks::HIDDEN, enabled)
	}

	/// Modifies the `AnsiStyle`'s foreground color, must be mutable
	pub fn m_fg(&mut self, foreground: Option<Color>) -> &mut Self {
		self.foreground = foreground;
		self
	}
	/// Modifies the `AnsiStyle`'s background color, must be mutable
	pub fn m_bg(&mut self, background: Option<Color>) -> &mut Self {
		self.background = background;
		self
	}

	/// Borrows and modifies the `AnsiStyle`'s bold flag
	pub fn bm_bold(self, enabled: bool) -> Self {
		self.b_set_flags(bit_masks::BOLD, enabled)
	}
	/// Borrows and modifies the `AnsiStyle`'s italic flag
	pub fn bm_italic(self, enabled: bool) -> Self {
		self.b_set_flags(bit_masks::ITALIC, enabled)
	}
	/// Borrows and modifies the `AnsiStyle`'s underline flag
	pub fn bm_underline(self, enabled: bool) -> Self {
		self.b_set_flags(bit_masks::UNDERLINE, enabled)
	}
	/// Borrows and modifies the `AnsiStyle`'s strikethrough flag
	pub fn bm_strikethrough(self, enabled: bool) -> Self {
		self.b_set_flags(bit_masks::STRIKETHROUGH, enabled)
	}
	/// Modifies the `AnsiStyle`'s dim flag, must be mutable
	pub fn bm_dim(self, enabled: bool) -> Self {
		self.b_set_flags(bit_masks::DIM, enabled)
	}
	/// Modifies the `AnsiStyle`'s blinking flag, must be mutable
	pub fn bm_blinking(self, enabled: bool) -> Self {
		self.b_set_flags(bit_masks::BLINKING, enabled)
	}
	/// Modifies the `AnsiStyle`'s reverse flag, must be mutable
	pub fn bm_reverse(self, enabled: bool) -> Self {
		self.b_set_flags(bit_masks::REVERSE, enabled)
	}
	/// Modifies the `AnsiStyle`'s hidden flag, must be mutable
	pub fn bm_hidden(self, enabled: bool) -> Self {
		self.b_set_flags(bit_masks::HIDDEN, enabled)
	}

	/// Borrows and modifies the `AnsiStyle`'s foreground color
	pub fn bm_fg(mut self, foreground: Option<Color>) -> Self {
		self.foreground = foreground;
		self
	}
	/// Borrows and modifies the `AnsiStyle`'s background color
	pub fn bm_bg(mut self, background: Option<Color>) -> Self {
		self.background = background;
		self
	}

	/// Modifies the passed `String` by applying the `AnsiStyle`
	pub fn m_apply(&self, text: &mut String) {
		text.insert_str(0, &self.to_string()[..]);
	}
	/// Modifies the passed `String` by applying the `AnsiStyle` with an `\x1b[0m` suffix
	pub fn m_apply_with_reset(&self, text: &mut String) {
		if self.is_plain() {
			return;
		}
		self.m_apply(text);
		text.push_str(codes::RESET);
	}

	/// Returns a new `AnsiStyle` with the bold flag set to `true`
	pub fn bold(&self) -> Self {
		Self {
			flags: self.flags | bit_masks::BOLD,
			..Default::default()
		}
	}
	/// Returns a new `AnsiStyle` with the italic flag set to `true`
	pub fn italic(&self) -> Self {
		Self {
			flags: self.flags | bit_masks::ITALIC,
			..Default::default()
		}
	}
	/// Returns a new `AnsiStyle` with the underline flag set to `true`
	pub fn underline(&self) -> Self {
		Self {
			flags: self.flags | bit_masks::UNDERLINE,
			..Default::default()
		}
	}
	/// Returns a new `AnsiStyle` with the strikethough flag set to `true`
	pub fn strikethrough(&self) -> Self {
		Self {
			flags: self.flags | bit_masks::STRIKETHROUGH,
			..Default::default()
		}
	}
	/// Modifies the `AnsiStyle`'s dim flag, must be mutable
	pub fn dim(&self) -> Self {
		Self {
			flags: self.flags | bit_masks::DIM,
			..Default::default()
		}
	}
	/// Modifies the `AnsiStyle`'s blinking flag, must be mutable
	pub fn blinking(&self) -> Self {
		Self {
			flags: self.flags | bit_masks::BLINKING,
			..Default::default()
		}
	}
	/// Modifies the `AnsiStyle`'s reverse flag, must be mutable
	pub fn reverse(&self) -> Self {
		Self {
			flags: self.flags | bit_masks::REVERSE,
			..Default::default()
		}
	}
	/// Modifies the `AnsiStyle`'s hidden flag, must be mutable
	pub fn hidden(&self) -> Self {
		Self {
			flags: self.flags | bit_masks::HIDDEN,
			..Default::default()
		}
	}

	/// Returns a new `AnsiStyle` with the foreground set the the specified `Color`
	pub fn fg(&self, foreground: Color) -> Self {
		Self {
			foreground: Some(foreground),
			..Default::default()
		}
	}
	/// Returns a new `AnsiStyle` with the background set the the specified `Color`
	pub fn bg(&self, background: Color) -> Self {
		Self {
			background: Some(background),
			..Default::default()
		}
	}

	/// Returns a cloned `String` of the passed in text with the `AnsiStyle` applied
	pub fn apply(&self, text: &str) -> String {
		let mut new_text = text.to_owned();
		self.m_apply(&mut new_text);
		new_text
	}
	/// Returns a cloned `String` of the passed in text with the `AnsiStyle` applied and an `\x1b[0m` suffix
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
		if self.g_dim() {
			append("2")?
		}
		if self.g_blinking() {
			append("5")?
		}
		if self.g_reverse() {
			append("7")?
		}
		if self.g_hidden() {
			append("8")?
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
		let expected = "\x1b[1mHello world!";

		assert_eq!(
			&text[..],
			"Hello world!" // "String doesn't equal Hello world?"
		);

		style.m_apply(&mut text);

		assert_eq!(
			&text[..],
			expected,
			"Expected: {:3?}\nGot: {:3?}",
			expected.as_bytes(),
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
			input_text // "Output text doesn't match input text"
		)
	}
}
