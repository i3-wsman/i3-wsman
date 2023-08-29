use std::fmt;

//pub enum Alignment {
//	Left,
//	Center,
//	Right
//}

#[derive(Debug, Clone, Default)]
pub struct Label {
	pub label: String,

	pub foreground: Option<String>,
	pub background: Option<String>,
	pub underline: Option<String>,
	pub overline: Option<String>,
	pub padding: Option<String>,
	pub margin: Option<String>,
	//pub maxlen: Option<u8>,
	//pub minlen: Option<u8>,
	//pub alignment: Option<Alignment>,
	//pub ellipsis: bool,
}

impl fmt::Display for Label {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut prefix = "".to_owned();
		let mut suffix = "".to_owned();

		if let Some(margin) = self.margin.clone() {
			prefix = prefix + format!("%{{O{}}}", margin).as_ref();
			suffix = format!("%{{O{}}}", margin) + suffix.as_ref();
		}

		if let Some(underline) = self.underline.clone() {
			prefix = prefix + format!("%{{u{}}}%{{+u}}", underline).as_ref();
			suffix = "%{-u}".to_string() + suffix.as_ref();
		}

		if let Some(overline) = self.overline.clone() {
			prefix = prefix + format!("%{{o{}}}%{{+o}}", overline).as_ref();
			suffix = "%{-o}".to_string() + suffix.as_ref();
		}

		if let Some(foreground) = self.foreground.clone() {
			prefix = prefix + format!("%{{F{}}}", foreground).as_ref();
			suffix = "%{F-}".to_string() + suffix.as_ref();
		}

		if let Some(background) = self.background.clone() {
			prefix = prefix + format!("%{{B{}}}", background).as_ref();
			suffix = "%{B-}".to_string() + suffix.as_ref();
		}

		if let Some(padding) = self.padding.clone() {
			prefix = prefix + format!("%{{O{}}}", padding).as_ref();
			suffix = format!("%{{O{}}}", padding) + suffix.as_ref();
		}

		write!(f, "{}{}{}", prefix, self.label, suffix)
	}
}
