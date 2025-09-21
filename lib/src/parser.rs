use crate::{
    expr::{Binary, Expr, Grouping, Literal, Unary},
    log::{debugging, log_error},
    token::{Token, TokenType},
};

struct Parser {
    tokens: Vec<Token>, // the input tokens to parse
    cur: usize,         // which token index we're currently looking at
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            cur: 0,
        }
    }

    // Scanning utils
    // --------------

    /// Whether we've reached the end of the token list
    fn finished(&mut self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    /// View the next token
    fn peek(&mut self) -> &Token {
        self.tokens
            .get(self.cur)
            .expect(log_error("Next token index is out of bounds".to_owned()).as_str())
    }

    /// View the last token
    fn previous(&mut self) -> &Token {
        self.tokens
            .get(self.cur - 1)
            .expect(log_error("Previous token index is out of bounds".to_owned()).as_str())
    }

    /// Consume and return the current token
    fn advance(&mut self) -> &Token {
        if !self.finished() {
            self.cur += 1;
        }

        self.previous()
    }

    /// Consume and return the current token if it matches the given type, otherwise returns an error
    fn expect(&mut self, token_type: TokenType, msg: &str) -> Result<&Token, String> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        let next = self.peek();
        Err(log_error(format!("Unexpected token [{}], {}", next, msg)))
    }

    /// Check if the current token is a given type
    fn check(&mut self, token_type: TokenType) -> bool {
        if self.finished() {
            return false;
        }

        self.peek().token_type == token_type
    }

    /// Check to see if the current token is any of the given types and advance if so
    fn match_types(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    // Grammar rules
    // -------------

    /// The topmost grammar rule for expressions
    fn expression(&mut self) -> Result<Box<dyn Expr>, String> {
        self.equality()
    }

    /// The grammar rule for equality expressions
    fn equality(&mut self) -> Result<Box<dyn Expr>, String> {
        let mut expr = self.comparison()?;

        while self.match_types(vec![TokenType::BangEquals, TokenType::EqualsEquals]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Binary::new(expr, operator, right);
        }

        Ok(expr)
    }

    /// The grammar rule for comparison expressions
    fn comparison(&mut self) -> Result<Box<dyn Expr>, String> {
        let mut expr = self.term()?;

        while self.match_types(vec![
            TokenType::RightAngle,
            TokenType::LeftAngle,
            TokenType::GreaterOrEquals,
            TokenType::LessOrEquals,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Binary::new(expr, operator, right);
        }

        Ok(expr)
    }

    /// The grammar rule for term expressions
    fn term(&mut self) -> Result<Box<dyn Expr>, String> {
        let mut expr = self.factor()?;

        while self.match_types(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Binary::new(expr, operator, right);
        }

        Ok(expr)
    }

    /// The grammar rule for factor expressions
    fn factor(&mut self) -> Result<Box<dyn Expr>, String> {
        let mut expr = self.unary()?;

        while self.match_types(vec![TokenType::Star, TokenType::Slash]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Binary::new(expr, operator, right);
        }

        Ok(expr)
    }

    /// The grammar rule for unary expressions
    fn unary(&mut self) -> Result<Box<dyn Expr>, String> {
        if self.match_types(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Unary::new(operator, right));
        }

        self.primary()
    }

    /// The grammar rule for primary expressions
    fn primary(&mut self) -> Result<Box<dyn Expr>, String> {
        if self.match_types(vec![
            TokenType::Underscore,
            TokenType::Int,
            TokenType::Float,
            TokenType::Char,
            TokenType::String,
            TokenType::FormString,
        ]) {
            return Ok(Literal::new(self.previous().clone()));
        }

        if self.match_types(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.expect(TokenType::RightParen, "Missing ')' after expression")?;
            return Ok(Grouping::new(expr));
        }

        let next = self.peek();
        Err(log_error(format!("Unexpected token: {}", next)))
    }
}

/// Parse tokens into an AST
pub fn parse(tokens: Vec<Token>, file_path: &String) -> Result<Box<dyn Expr>, String> {
    let mut parser: Parser = Parser::new(tokens);
    let expr: Box<dyn Expr>;

    match parser.expression() {
        Ok(res) => {
            expr = res;
        }
        Err(_) => {
            return Err(format!("syntax error encountered in {}", file_path));
        }
    }

    // Print the syntax tree if debugging
    if debugging() {
        
    }

    Ok(expr)
}
