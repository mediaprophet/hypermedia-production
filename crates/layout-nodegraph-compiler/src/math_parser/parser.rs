//! Recursive-descent precedence climbing parser with implicit multiplication support.

#![forbid(unsafe_code)]

use super::ast::{BinaryOp, MathNode, UnaryOp, Unit};
use super::lexer::{Token, tokenize};

pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<&Token> {
        let t = self.tokens.get(self.pos);
        if t.is_some() {
            self.pos += 1;
        }
        t
    }

    pub fn parse_expression(&mut self) -> Result<MathNode, String> {
        self.parse_precedence(1)
    }

    fn parse_precedence(&mut self, min_prec: u8) -> Result<MathNode, String> {
        let mut left = self.parse_primary()?;

        loop {
            // Check for implicit multiplication (e.g. `2pi`, `5(3+1)`, `10px`)
            if let Some(next_tok) = self.peek() {
                let is_implicit = matches!(next_tok, Token::Number(_) | Token::Ident(_) | Token::LParen);
                if is_implicit && min_prec <= 3 {
                    let right = self.parse_precedence(4)?;
                    left = MathNode::Binary(BinaryOp::Mul, Box::new(left), Box::new(right));
                    continue;
                }
            }

            let Some(op) = self.peek_binary_op() else { break };
            let (l_prec, r_prec) = op_precedence(op);
            if l_prec < min_prec {
                break;
            }

            self.pos += 1; // consume op
            let right = self.parse_precedence(r_prec)?;
            left = MathNode::Binary(op, Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn peek_binary_op(&self) -> Option<BinaryOp> {
        match self.peek()? {
            Token::Plus => Some(BinaryOp::Add),
            Token::Minus => Some(BinaryOp::Sub),
            Token::Star => Some(BinaryOp::Mul),
            Token::Slash => Some(BinaryOp::Div),
            Token::Percent => Some(BinaryOp::Mod),
            Token::Caret => Some(BinaryOp::Pow),
            _ => None,
        }
    }

    fn parse_primary(&mut self) -> Result<MathNode, String> {
        match self.next() {
            Some(Token::Plus) => {
                let expr = self.parse_primary()?;
                Ok(MathNode::Unary(UnaryOp::Pos, Box::new(expr)))
            }
            Some(Token::Minus) => {
                let expr = self.parse_primary()?;
                Ok(MathNode::Unary(UnaryOp::Neg, Box::new(expr)))
            }
            Some(Token::Number(val)) => {
                let v = *val;
                // Check if followed by unit ident
                if let Some(Token::Ident(name)) = self.peek() {
                    if let Some(u) = Unit::parse(name) {
                        self.pos += 1;
                        return Ok(MathNode::UnitValue(v, u));
                    }
                }
                Ok(MathNode::Number(v))
            }
            Some(Token::Ident(name)) => {
                let name = name.clone();
                if self.peek() == Some(&Token::LParen) {
                    self.pos += 1; // consume '('
                    let mut args = Vec::new();
                    if self.peek() != Some(&Token::RParen) {
                        loop {
                            args.push(self.parse_expression()?);
                            if self.peek() == Some(&Token::Comma) {
                                self.pos += 1;
                            } else {
                                break;
                            }
                        }
                    }
                    if self.next() != Some(&Token::RParen) {
                        return Err(format!("Expected ')' after function `{name}` arguments"));
                    }
                    Ok(MathNode::FunctionCall(name, args))
                } else {
                    Ok(MathNode::Variable(name))
                }
            }
            Some(Token::LParen) => {
                let expr = self.parse_expression()?;
                if self.next() != Some(&Token::RParen) {
                    return Err("Expected ')' matching '('".into());
                }
                Ok(expr)
            }
            Some(other) => Err(format!("Unexpected token `{other:?}`")),
            None => Err("Unexpected end of expression".into()),
        }
    }
}

fn op_precedence(op: BinaryOp) -> (u8, u8) {
    match op {
        BinaryOp::Add | BinaryOp::Sub => (1, 2),
        BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => (3, 4),
        BinaryOp::Pow => (6, 5), // right-associative
    }
}

pub fn parse(s: &str) -> Result<MathNode, String> {
    let tokens = tokenize(s)?;
    let mut p = Parser::new(&tokens);
    let ast = p.parse_expression()?;
    if p.pos != tokens.len() {
        return Err("Trailing tokens after expression".into());
    }
    Ok(ast)
}
