use crate::token::Token;

/// Implement a visitor pattern for expressions
pub trait Expr {
    fn accept(&self, visitor: &mut dyn Visitor);
}
trait Visitor {
    fn visit_assign(&mut self, expr: &Assign);
    fn visit_binary(&mut self, expr: &Binary);
    fn visit_unary(&mut self, expr: &Unary);
    fn visit_literal(&mut self, expr: &Literal);
    fn visit_grouping(&mut self, expr: &Grouping);
}

/// An assignment expression
pub struct Assign {
    name: Token,
    value: Box<dyn Expr>,
}
impl Expr for Assign {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_assign(self);
    }
}

/// A binary expression
pub struct Binary {
    left: Box<dyn Expr>,
    operator: Token,
    right: Box<dyn Expr>,
}
impl Binary {
    pub fn new(left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>) -> Box<Binary> {
        Box::new(Binary {
            left: left,
            operator: operator,
            right: right,
        })
    }
}
impl Expr for Binary {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_binary(self);
    }
}

/// A unary expression
pub struct Unary {
    operator: Token,
    right: Box<dyn Expr>,
}
impl Unary {
    pub fn new(operator: Token, right: Box<dyn Expr>) -> Box<Unary> {
        Box::new(Unary {
            operator: operator,
            right: right,
        })
    }
}
impl Expr for Unary {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_unary(self);
    }
}

/// A literal expression
pub struct Literal {
    value: Token,
}
impl Literal {
    pub fn new(value: Token) -> Box<Literal> {
        Box::new(Literal { value: value })
    }
}
impl Expr for Literal {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_literal(self);
    }
}

/// A grouping expression
pub struct Grouping {
    expr: Box<dyn Expr>,
}
impl Grouping {
    pub fn new(expr: Box<dyn Expr>) -> Box<Grouping> {
        Box::new(Grouping { expr: expr })
    }
}
impl Expr for Grouping {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_grouping(self);
    }
}

// Call
// Get
// Logical
// Set
// Super
// This
// Variable
