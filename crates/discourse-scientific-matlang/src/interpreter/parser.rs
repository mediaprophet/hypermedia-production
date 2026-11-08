//! Recursive descent parser for M-code grammar.

use crate::interpreter::ast::{BinOp, Expr, Stmt, UnOp};
use crate::interpreter::lexer::{LangError, Tok};

pub struct Parser {
    toks: Vec<Tok>,
    pos: usize,
}

impl Parser {
    pub fn new(toks: Vec<Tok>) -> Self {
        Self { toks, pos: 0 }
    }

    fn peek(&self) -> &Tok {
        self.toks.get(self.pos).unwrap_or(&Tok::Eof)
    }

    fn next(&mut self) -> Tok {
        if self.pos < self.toks.len() {
            let t = self.toks[self.pos].clone();
            self.pos += 1;
            t
        } else {
            Tok::Eof
        }
    }

    fn skip_newlines_and_semicolons(&mut self) {
        while matches!(self.peek(), Tok::Newline | Tok::Semicolon | Tok::Comma) {
            self.next();
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, LangError> {
        let mut stmts = Vec::new();
        self.skip_newlines_and_semicolons();
        while self.peek() != &Tok::Eof {
            stmts.push(self.statement()?);
            self.skip_newlines_and_semicolons();
        }
        Ok(stmts)
    }

    fn statement(&mut self) -> Result<Stmt, LangError> {
        match self.peek() {
            Tok::Ident(id) if id == "if" => self.parse_if(),
            Tok::Ident(id) if id == "for" => self.parse_for(),
            Tok::Ident(id) if id == "while" => self.parse_while(),
            Tok::Ident(id) if id == "return" => {
                self.next();
                Ok(Stmt::Return)
            }
            Tok::Ident(name) if self.pos + 1 < self.toks.len() && self.toks[self.pos + 1] == Tok::Eq => {
                let name = name.clone();
                self.next(); // name
                self.next(); // =
                let expr = self.expression()?;
                let suppress = if self.peek() == &Tok::Semicolon {
                    self.next();
                    true
                } else {
                    false
                };
                Ok(Stmt::Assign {
                    name,
                    value: expr,
                    suppress,
                })
            }
            _ => {
                let expr = self.expression()?;
                let suppress = if self.peek() == &Tok::Semicolon {
                    self.next();
                    true
                } else {
                    false
                };
                Ok(Stmt::Expr(expr, suppress))
            }
        }
    }

    fn parse_if(&mut self) -> Result<Stmt, LangError> {
        self.next(); // 'if'
        let cond = self.expression()?;
        self.skip_newlines_and_semicolons();

        let mut then_body = Vec::new();
        let mut else_body = None;

        while self.peek() != &Tok::Eof {
            if let Tok::Ident(kw) = self.peek() {
                if kw == "end" {
                    self.next();
                    break;
                }
                if kw == "else" {
                    self.next();
                    self.skip_newlines_and_semicolons();
                    let mut eb = Vec::new();
                    while self.peek() != &Tok::Eof {
                        if let Tok::Ident(k) = self.peek() {
                            if k == "end" {
                                self.next();
                                break;
                            }
                        }
                        eb.push(self.statement()?);
                        self.skip_newlines_and_semicolons();
                    }
                    else_body = Some(eb);
                    break;
                }
            }
            then_body.push(self.statement()?);
            self.skip_newlines_and_semicolons();
        }

        Ok(Stmt::If {
            cond,
            then_body,
            else_body,
        })
    }

    fn parse_for(&mut self) -> Result<Stmt, LangError> {
        self.next(); // 'for'
        let var = match self.next() {
            Tok::Ident(id) => id,
            _ => return Err(LangError { message: "expected loop variable".into(), line: 0 }),
        };
        if self.next() != Tok::Eq {
            return Err(LangError { message: "expected '=' in for loop".into(), line: 0 });
        }
        let iter = self.expression()?;
        self.skip_newlines_and_semicolons();

        let mut body = Vec::new();
        while self.peek() != &Tok::Eof {
            if let Tok::Ident(kw) = self.peek() {
                if kw == "end" {
                    self.next();
                    break;
                }
            }
            body.push(self.statement()?);
            self.skip_newlines_and_semicolons();
        }

        Ok(Stmt::For { var, iter, body })
    }

    fn parse_while(&mut self) -> Result<Stmt, LangError> {
        self.next(); // 'while'
        let cond = self.expression()?;
        self.skip_newlines_and_semicolons();

        let mut body = Vec::new();
        while self.peek() != &Tok::Eof {
            if let Tok::Ident(kw) = self.peek() {
                if kw == "end" {
                    self.next();
                    break;
                }
            }
            body.push(self.statement()?);
            self.skip_newlines_and_semicolons();
        }

        Ok(Stmt::While { cond, body })
    }

    fn expression(&mut self) -> Result<Expr, LangError> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<Expr, LangError> {
        let mut left = self.parse_logical_and()?;
        while self.peek() == &Tok::Pipe {
            self.next();
            let right = self.parse_logical_and()?;
            left = Expr::Binary(BinOp::Or, Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_logical_and(&mut self) -> Result<Expr, LangError> {
        let mut left = self.parse_comparison()?;
        while self.peek() == &Tok::Amp {
            self.next();
            let right = self.parse_comparison()?;
            left = Expr::Binary(BinOp::And, Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, LangError> {
        let mut left = self.parse_range()?;
        let op = match self.peek() {
            Tok::EqEq => Some(BinOp::Eq),
            Tok::Neq => Some(BinOp::Neq),
            Tok::Lt => Some(BinOp::Lt),
            Tok::Le => Some(BinOp::Le),
            Tok::Gt => Some(BinOp::Gt),
            Tok::Ge => Some(BinOp::Ge),
            _ => None,
        };
        if let Some(op) = op {
            self.next();
            let right = self.parse_range()?;
            left = Expr::Binary(op, Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_range(&mut self) -> Result<Expr, LangError> {
        let left = self.parse_add_sub()?;
        if self.peek() == &Tok::Colon {
            self.next();
            let mid_or_end = self.parse_add_sub()?;
            if self.peek() == &Tok::Colon {
                self.next();
                let end = self.parse_add_sub()?;
                Ok(Expr::Range(Box::new(left), Some(Box::new(mid_or_end)), Box::new(end)))
            } else {
                Ok(Expr::Range(Box::new(left), None, Box::new(mid_or_end)))
            }
        } else {
            Ok(left)
        }
    }

    fn parse_add_sub(&mut self) -> Result<Expr, LangError> {
        let mut left = self.parse_mul_div()?;
        while matches!(self.peek(), Tok::Plus | Tok::Minus) {
            let op = match self.next() {
                Tok::Plus => BinOp::Add,
                _ => BinOp::Sub,
            };
            let right = self.parse_mul_div()?;
            left = Expr::Binary(op, Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_mul_div(&mut self) -> Result<Expr, LangError> {
        let mut left = self.parse_power()?;
        while matches!(self.peek(), Tok::Star | Tok::Slash | Tok::DotStar | Tok::DotSlash) {
            let op = match self.next() {
                Tok::Star => BinOp::Mul,
                Tok::Slash => BinOp::Div,
                Tok::DotStar => BinOp::ElemMul,
                _ => BinOp::ElemDiv,
            };
            let right = self.parse_power()?;
            left = Expr::Binary(op, Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_power(&mut self) -> Result<Expr, LangError> {
        let left = self.parse_unary()?;
        if matches!(self.peek(), Tok::Caret | Tok::DotCaret) {
            let op = match self.next() {
                Tok::Caret => BinOp::Pow,
                _ => BinOp::ElemPow,
            };
            let right = self.parse_power()?;
            Ok(Expr::Binary(op, Box::new(left), Box::new(right)))
        } else {
            Ok(left)
        }
    }

    fn parse_unary(&mut self) -> Result<Expr, LangError> {
        if self.peek() == &Tok::Minus {
            self.next();
            let operand = self.parse_unary()?;
            Ok(Expr::Unary(UnOp::Neg, Box::new(operand)))
        } else if self.peek() == &Tok::Tilde {
            self.next();
            let operand = self.parse_unary()?;
            Ok(Expr::Unary(UnOp::Not, Box::new(operand)))
        } else {
            self.parse_postfix()
        }
    }

    fn parse_postfix(&mut self) -> Result<Expr, LangError> {
        let mut expr = self.parse_primary()?;
        while self.peek() == &Tok::Apostrophe {
            self.next();
            expr = Expr::Transpose(Box::new(expr));
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, LangError> {
        match self.next() {
            Tok::Num(n) => Ok(Expr::Num(n)),
            Tok::Str(s) => Ok(Expr::Str(s)),
            Tok::Ident(id) => {
                if self.peek() == &Tok::LParen {
                    self.next(); // '('
                    let mut args = Vec::new();
                    if self.peek() != &Tok::RParen {
                        args.push(self.expression()?);
                        while self.peek() == &Tok::Comma {
                            self.next();
                            args.push(self.expression()?);
                        }
                    }
                    if self.next() != Tok::RParen {
                        return Err(LangError { message: "expected ')'".into(), line: 0 });
                    }
                    Ok(Expr::CallOrIndex(id, args))
                } else {
                    Ok(Expr::Ident(id))
                }
            }
            Tok::LParen => {
                let e = self.expression()?;
                if self.next() != Tok::RParen {
                    return Err(LangError { message: "expected ')'".into(), line: 0 });
                }
                Ok(e)
            }
            Tok::LBracket => self.parse_matrix_literal(),
            other => Err(LangError {
                message: format!("unexpected token in expression: {:?}", other),
                line: 0,
            }),
        }
    }

    fn parse_matrix_literal(&mut self) -> Result<Expr, LangError> {
        let mut rows = Vec::new();
        let mut current_row = Vec::new();

        while self.peek() != &Tok::RBracket && self.peek() != &Tok::Eof {
            if matches!(self.peek(), Tok::Semicolon | Tok::Newline) {
                self.next();
                if !current_row.is_empty() {
                    rows.push(current_row);
                    current_row = Vec::new();
                }
                continue;
            }
            if self.peek() == &Tok::Comma {
                self.next();
                continue;
            }
            current_row.push(self.expression()?);
        }

        if self.next() != Tok::RBracket {
            return Err(LangError { message: "expected ']' in matrix literal".into(), line: 0 });
        }
        if !current_row.is_empty() {
            rows.push(current_row);
        }

        Ok(Expr::Matrix(rows))
    }
}
