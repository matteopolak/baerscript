#[derive(Debug, PartialEq)]
pub enum Token {
	Null,
	Add,      // +
	Multiply, // *
	Subtract, // -
	Left,     // <
	Right,    // >
	Up,       // ^
	Down,     // v
	Comment,  // #
	BlankSpace,
	OpenSquareBracket,   // [
	ClosedSquareBracket, // ]
}

/// Create conversion from char -> Token
impl TryFrom<char> for Token {
	type Error = String;

	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value {
			'.' => Ok(Self::Null),
			'+' => Ok(Self::Add),
			'-' => Ok(Self::Subtract),
			'*' => Ok(Self::Multiply),
			'<' => Ok(Self::Left),
			'>' => Ok(Self::Right),
			'^' => Ok(Self::Up),
			'v' => Ok(Self::Down),
			'#' => Ok(Self::Comment),
			'\u{0032}' | '\u{0009}' => Ok(Self::BlankSpace),
			'[' => Ok(Self::OpenSquareBracket),
			']' => Ok(Self::ClosedSquareBracket),
			c @ _ => Err(format!("Unknown character {c}")),
		}
	}
}

// Implement Display trait for Token so it can be printed and
// turned into a string (e.g. Token::NULL.to_string())
impl std::fmt::Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Token::Null => " ",
				Token::Add => "+",
				Token::Subtract => "-",
				Token::Multiply => "*",
				Token::Left => "<",
				Token::Right => ">",
				Token::Up => "^",
				Token::Down => "v",
				Token::Comment => "#",
				Token::BlankSpace => " ",
				Token::OpenSquareBracket => "[",
				Token::ClosedSquareBracket => "]",
			}
		)
	}
}
