use std::fmt;

use colored::Colorize;

#[derive(strum_macros::Display, PartialEq)]
pub enum TokenType {
    // Single-char tokens
    Hash,         // #
    Dollar,       // $
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }
    Comma,        // ,
    Question,     // ?
    At,           // @
    Underscore,   // _

    // Single or double-char tokens by initial char
    Bang,          // !
    BangEquals,    // !=
    Percent,       // %
    PercentEquals, // %=
    And,           // &
    AndAnd,        // &&
    AndEquals,     // &=
    Star,          // *
    StarEquals,    // *=
    Plus,          // +
    PlusEquals,    // +=
    Minus,         // -
    MinusEquals,   // -=
    RightArrow,    // ->
    Slash,         // /
    SlashEquals,   // /=
    Colon,         // :
    ColonEquals,   // :=
    ColonColon,    // ::
    Equals,        // =
    EqualsEquals,  // ==
    EqualsArrow,   // =>
    Caret,         // ^
    CaretEquals,   // ^=
    Pipe,          // |
    PipePipe,      // ||
    Tilde,         // ~
    TildeEquals,   // ~=

    // Single, double, or triple-char tokens by initial char
    LeftAngle,        // <
    LeftShift,        // <<
    LeftArrow,        // <-
    LessOrEquals,     // <=
    LeftShiftEquals,  // <<=
    RightAngle,       // >
    GreaterOrEquals,  // >=
    RightShift,       // >>
    RightShiftEquals, // >>=
    Dot,              // .
    DotQuestion,      // .?
    DotDotQuestion,   // ..?

    // Literals
    Identifier, // begins with a-zA-Z
    String,     // begins with "
    FormString, // begins with `
    Char,       // begins with '
    Int,        // sequence of only 0-9
    Float,      // sequence of only 0-9 and exactly 1 non-initial, non-final .

    // Other
    Comment, // ; until end of line
    EOF,     // end of file
}

/// Returns the matching TokenType for a string representation of an operator, if applicable
pub fn parse_op(op: &str) -> Option<TokenType> {
    match op {
        // Single char
        "#" => Some(TokenType::Hash),
        "$" => Some(TokenType::Dollar),
        "(" => Some(TokenType::LeftParen),
        ")" => Some(TokenType::RightParen),
        "[" => Some(TokenType::LeftBracket),
        "]" => Some(TokenType::RightBracket),
        "{" => Some(TokenType::LeftBrace),
        "}" => Some(TokenType::RightBrace),
        "," => Some(TokenType::Comma),
        "?" => Some(TokenType::Question),
        "@" => Some(TokenType::At),
        "_" => Some(TokenType::Underscore),

        // Single and double-char
        "~=" => Some(TokenType::TildeEquals),
        "~" => Some(TokenType::Tilde),
        "||" => Some(TokenType::PipePipe),
        "|" => Some(TokenType::Pipe),
        "^=" => Some(TokenType::CaretEquals),
        "^" => Some(TokenType::Caret),
        "=>" => Some(TokenType::EqualsArrow),
        "==" => Some(TokenType::EqualsEquals),
        "=" => Some(TokenType::Equals),
        ":=" => Some(TokenType::ColonEquals),
        "::" => Some(TokenType::ColonColon),
        ":" => Some(TokenType::Colon),
        "/=" => Some(TokenType::SlashEquals),
        "/" => Some(TokenType::Slash),
        "->" => Some(TokenType::RightArrow),
        "-=" => Some(TokenType::MinusEquals),
        "-" => Some(TokenType::Minus),
        "+=" => Some(TokenType::PlusEquals),
        "+" => Some(TokenType::Plus),
        "*=" => Some(TokenType::StarEquals),
        "*" => Some(TokenType::Star),
        "&=" => Some(TokenType::AndEquals),
        "&&" => Some(TokenType::AndAnd),
        "&" => Some(TokenType::And),
        "%=" => Some(TokenType::PercentEquals),
        "%" => Some(TokenType::Percent),
        "!=" => Some(TokenType::BangEquals),
        "!" => Some(TokenType::Bang),

        // Single, double and triple-char
        "<<=" => Some(TokenType::LeftShiftEquals),
        "<<" => Some(TokenType::LeftShift),
        "<=" => Some(TokenType::LessOrEquals),
        "<-" => Some(TokenType::LeftArrow),
        "<" => Some(TokenType::LeftAngle),
        ">>=" => Some(TokenType::RightShiftEquals),
        ">>" => Some(TokenType::RightShift),
        ">=" => Some(TokenType::GreaterOrEquals),
        ">" => Some(TokenType::RightAngle),
        "..?" => Some(TokenType::DotDotQuestion),
        ".?" => Some(TokenType::DotQuestion),
        "." => Some(TokenType::Dot),

        _ => None,
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: i32,
    pub col: i32,
}

impl Token {
    /// Create a new Token
    pub fn new(value: String, token_type: TokenType, line: i32, col: i32) -> Token {
        Token {
            token_type: token_type,
            value: value,
            line: line,
            col: col,
        }
    }
}

/// Allow pretty-printing of Tokens
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} ({}, {})",
            self.value.bold(), format!("{}", self.token_type).italic(), self.line, self.col
        )
    }
}
