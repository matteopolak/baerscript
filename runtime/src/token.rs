#[derive(Debug, PartialEq)]
pub enum Token {
	NULL,
	ADD,      // +
	MULTIPLY, // *
	SUBTRACT, // -
	LEFT,     // <
	RIGHT,    // >
	UP,       // ^
	DOWN,     // v
	COMMENT,  // #
	BLANKSPACE,
}

/// Create conversion from char -> Token
impl TryFrom<char> for Token {
	type Error = String;

	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value {
			'.' => Ok(Self::NULL),
			'+' => Ok(Self::ADD),
			'-' => Ok(Self::SUBTRACT),
			'*' => Ok(Self::MULTIPLY),
			'<' => Ok(Self::LEFT),
			'>' => Ok(Self::RIGHT),
			'^' => Ok(Self::UP),
			'v' => Ok(Self::DOWN),
			'#' => Ok(Self::COMMENT),
			'\u{0032}' | '\u{0009}' => Ok(Self::BLANKSPACE),
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
				Token::NULL => " ",
				Token::ADD => "+",
				Token::SUBTRACT => "-",
				Token::MULTIPLY => "*",
				Token::LEFT => "<",
				Token::RIGHT => ">",
				Token::UP => "^",
				Token::DOWN => "v",
				Token::COMMENT => "#",
				Token::BLANKSPACE => " ",
			}
		)
	}
}
