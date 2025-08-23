use std::{fmt, path::PathBuf, vec::Vec};

#[derive(strum_macros::Display)]
pub enum TokenType {
    Eof, // End of file
    Keyword,
    Operator,
    Identifier,
    Quality,  // Boolean literal
    WholeNum, // Integer literal
    PointNum, // Floating point literal
    Text,     // String literal
    FormText, // Formatted string literal
    Comment,
}

pub struct Token {
    token_type: TokenType,
    value: String,
    line: i32,
    col: i32,
}

impl Token {
    pub fn new(value: &str, token_type: TokenType) -> Token {
        Token {
            token_type: token_type,
            value: value.to_string(),
            line: 3,
            col: 1,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Token: type={}, value={}, line={}, col={}",
            self.token_type, self.value, self.line, self.col
        )
    }
}

pub fn tokenize(src_file: PathBuf) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();

    println!("Tokenizing '{}'...", src_file.display());

    Ok(tokens)
}
