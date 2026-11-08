//! Pure-Rust M-code interpreter module.

pub mod ast;
pub mod eval;
pub mod lexer;
pub mod parser;

pub use eval::{Value, Workspace};
pub use lexer::{lex, LangError};
pub use parser::Parser;

pub fn eval_m_code(source: &str) -> Result<Workspace, LangError> {
    let toks = lex(source)?;
    let mut parser = Parser::new(toks);
    let stmts = parser.parse()?;
    let mut ws = Workspace::new();
    ws.exec(&stmts)?;
    Ok(ws)
}
