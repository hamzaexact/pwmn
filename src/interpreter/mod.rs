pub mod ast;
pub mod lexer;
pub mod parser;
pub use crate::error::LexerErr;

pub use lexer::{Span,Token};


pub fn push_token(collector: &mut Vec<Token> ,tTokind: lexer::TokenKind, start:usize, end:usize) {
    collector.push(
        Token {
            kind: tTokind,
            span: Span {
                start,
                end
            }
        }
    );
}
