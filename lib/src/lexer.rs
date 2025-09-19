use std::vec::Vec;

use crate::{
    log::{debugging, log_debug, log_error},
    token::{Token, TokenType, parse_op},
};

struct Lexer {
    input: Vec<char>, // split input source code into individual chars
    pos: usize,       // current scan position
    next: usize,      // next scan position
    cur: char,        // the char we're currently looking at
    line: i32,        // what line in the source code we're at
    col: i32,         // what column in the source code we're at
    error_count: i32, // how many scan errors we've had
}

impl Lexer {
    /// Create a new Lexer
    fn new(source_code: String) -> Lexer {
        let mut lexer = Lexer {
            input: source_code.chars().collect(),
            pos: 0,
            next: 0,
            cur: '\u{0}',
            line: 1,
            col: 0,
            error_count: 0,
        };

        lexer.read_char();
        lexer
    }

    /// Get the next Token
    fn next_token(&mut self) -> Token {
        let empty_token = Token::new("".to_owned(), TokenType::EOF, self.line, self.col);
        self.skip_whitespace();

        // Reached end of input, return the empty token
        if self.cur == '\u{0}' {
            return empty_token;
        }

        // Scan tokens based on the type of initial char
        // ---------------------------------------------

        if is_letter(self.cur) {
            let value = self.read_identifier();
            return self.create_token(TokenType::Identifier, value);
        }

        if is_digit(self.cur) {
            let (token_type, value) = self.read_number();
            return self.create_token(token_type, value);
        }

        if is_quote(self.cur) {
            let (token_type, value) = self.read_quote();
            return self.create_token(token_type, value);
        }

        match parse_op(self.cur.to_string().as_str()) {
            Some(op) => {
                let (token_type, value) = self.read_operator(op);
                return self.create_token(token_type, value);
            }
            None => {}
        }

        // comment opening
        if self.cur == ';' {
            let value = self.read_comment();
            return self.create_token(TokenType::Comment, value);
        }

        log_error(format!(
            "Unrecognized character [{}] on line {} col {}",
            self.cur, self.line, self.col
        ));
        self.error_count += 1;
        self.read_char();
        empty_token
    }

    /// Create a token with the given parameters
    fn create_token(&mut self, token_type: TokenType, value: String) -> Token {
        // If we're creating a token enclosed in quotes, subtract the quote positions from the token's location
        let mut col_offs = (&value).len() as i32;
        if token_type == TokenType::String
            || token_type == TokenType::FormString
            || token_type == TokenType::Char
        {
            col_offs += 2;
        }

        Token::new(value, token_type, self.line, self.col - col_offs)
    }

    /// Read over whitespace
    fn skip_whitespace(&mut self) {
        while is_whitespace(self.cur) {
            if self.cur == '\n' {
                self.line += 1;
                self.col = 0;
            }

            self.read_char();
        }
    }

    /// Read the character at the next position, then advance the position
    fn read_char(&mut self) {
        if self.next >= self.input.len() {
            // End of input
            self.cur = '\u{0}';
        } else {
            self.cur = self.input[self.next];
        }

        self.pos = self.next;
        self.next += 1;
        self.col += 1;
    }

    /// Read an identifier
    fn read_identifier(&mut self) -> String {
        let start: usize = self.pos;
        while is_letter(self.cur) {
            self.read_char();
        }

        self.input[start..self.pos].iter().collect()
    }

    /// Read a number literal (integer or floating point)
    fn read_number(&mut self) -> (TokenType, String) {
        let start: usize = self.pos;
        let mut token_type = TokenType::Int;

        while is_digit(self.cur) {
            self.read_char();

            // If we encounter a dot, we should only keep reading if the following char is a digit
            if self.cur == '.' && self.next < self.input.len() && is_digit(self.input[self.next]) {
                token_type = TokenType::Float;
                self.read_char();
            }
        }

        (token_type, self.input[start..self.pos].iter().collect())
    }

    /// Read a quote; scans until the closing quote is found.
    /// If it scans until EOF without finding a close, we have an error.
    fn read_quote(&mut self) -> (TokenType, String) {
        let quote: char = self.cur;
        let line: i32 = self.line;
        let col: i32 = self.col;
        let mut token_type = TokenType::String;

        if quote == '\'' {
            token_type = TokenType::Char;
        } else if quote == '`' {
            token_type = TokenType::FormString;
        }

        self.read_char();
        let start: usize = self.pos;
        let mut closed = true;

        // Look back 2 chars to check if we're escaping the next char. Account for escaped backslash
        let mut last = '\u{0}';
        let mut last2 = '\u{0}';

        while self.cur != quote || (last == '\\' && last2 != '\\') {
            last2 = last;
            last = self.cur;
            self.read_char();

            // If we get to the end of the file without closing the string, log an error
            if self.cur == '\u{0}' {
                log_error(format!(
                    "Unclosed quote [{}] on line {} col {}",
                    quote, line, col
                ));
                self.error_count += 1;
                closed = false;
                break;
            }
        }

        let end: usize = self.pos;
        let buffer: String = self.input[start..end].iter().collect();

        // If we've read a char literal, check that it is valid
        if closed && quote == '\'' && !is_valid_char(&buffer) {
            log_error(format!(
                "Invalid char literal [{}] on line {} col {}",
                buffer, line, col
            ));
            self.error_count += 1;
        }

        self.read_char();
        (token_type, buffer)
    }

    /// Read a comment: comments are from the opening char until the end of the line (or EOF)
    fn read_comment(&mut self) -> String {
        let start: usize = self.pos;
        while self.cur != '\n' && self.cur != '\u{0}' {
            self.read_char();
        }

        self.input[start..self.pos].iter().collect()
    }

    /// Read an operator: an operator can be 1, 2, or 3 chars long
    fn read_operator(&mut self, base: TokenType) -> (TokenType, String) {
        let mut token_type = base;
        let mut buffer: String = self.cur.to_string();
        let mut to_scan: usize = 0;

        // Check up to the 3 chars from the current position if we can
        for offs in (1..=3).rev() {
            if self.pos + offs <= self.input.len() {
                let op: String = self.input[self.pos..self.pos + offs].iter().collect();
                match parse_op(op.as_str()) {
                    Some(op_type) => {
                        token_type = op_type;
                        buffer = op;
                        to_scan += offs;
                        break;
                    }
                    None => {}
                }
            }
        }

        //  Catch up the internal pointer
        for _ in 0..to_scan {
            self.read_char();
        }

        (token_type, buffer)
    }
}

/// Convert raw Rumil source code text into a vector of tokens.
/// If there was an error, return an error message instead
pub fn scan(source_code: String, file_path: &String) -> Result<Vec<Token>, String> {
    let mut lexer: Lexer = Lexer::new(source_code);
    let mut tokens: Vec<Token> = Vec::new();
    let mut token: Token = lexer.next_token();

    // Scan tokens
    while token.token_type != TokenType::EOF {
        tokens.push(token);
        token = lexer.next_token();
    }

    // Report errors
    if tokens.len() == 0 {
        return Err(format!("No source code found in {}", file_path));
    }
    if lexer.error_count > 0 {
        let mut s: &str = "";
        if lexer.error_count > 1 {
            s = "s";
        }

        return Err(format!(
            "{} syntax error{} encountered in {}",
            lexer.error_count, s, file_path
        ));
    }

    // Log the tokens if debugging
    if debugging() {
        let mut token_strings = String::new();
        tokens.iter().for_each(|tk| token_strings.push_str(format!("    {}\n", tk).as_str()));
        log_debug(format!("Tokens:\n{}", token_strings))
    }

    Ok(tokens)
}

// Scanning utils
// --------------

/// Utility function to tell us if a char is whitespace
fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\n' || c == '\r' || c == '\t'
}

/// Utility function to tell us if a char is a letter
fn is_letter(c: char) -> bool {
    ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z')
}

/// Utility function to tell us if a char is a digit
fn is_digit(c: char) -> bool {
    '0' <= c && c <= '9'
}

/// Utility function to tell us if a char is a quotation mark
fn is_quote(c: char) -> bool {
    c == '"' || c == '\'' || c == '`'
}

/// Utility function to tell us if a string is a valid char literal
fn is_valid_char(c: &str) -> bool {
    if c.len() == 1 && c != "\\" {
        return true;
    }

    is_valid_unicode_point(c) || is_valid_escaped_char(c)
}

/// Utility function to tell us if a literal is a valid unicode point
fn is_valid_unicode_point(u: &str) -> bool {
    if !u.starts_with("\\u") || u.len() == 2 || u.len() > 10 {
        return false;
    }

    // Each digit past the first 2 must be a valid hexadecimal digit
    let valid = |x: char| -> bool {
        (x >= '0' && x <= '9') || (x >= 'A' && x <= 'F') || (x >= 'a' && x <= 'f')
    };

    for i in 2..u.len() {
        if let Some(x) = u.chars().nth(i)
            && !valid(x)
        {
            return false;
        }
    }

    return true;
}

/// Utility function to tell us if a literal is a valid escape sequence
fn is_valid_escaped_char(e: &str) -> bool {
    match e {
        // Backslash
        r"\\" => true,

        // Alert
        r"\a" => true,

        // Backspace
        r"\b" => true,

        // Page break (form feed)
        r"\f" => true,

        // Newline (line feed)
        r"\n" => true,

        // Carriage return
        r"\r" => true,

        // Horizontal tab
        r"\t" => true,

        // Vertical tab
        r"\v" => true,

        // Single quote
        r"\'" => true,

        // Null char
        r"\0" => true,

        // Everything else is invalid
        _ => false,
    }
}
