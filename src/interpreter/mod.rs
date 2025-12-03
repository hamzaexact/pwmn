pub mod ast;
pub mod eval;
pub mod lexer;
pub mod parser;
pub use crate::error::LexerErr;
use lexer::TokenKind;

pub use lexer::{Span, Token};

pub fn push_token(collector: &mut Vec<Token>, tTokind: lexer::TokenKind, start: usize, end: usize) {
    collector.push(Token {
        kind: tTokind,
        span: Span { start, end },
    });
}

pub fn token_name(tokind: &TokenKind) -> &str {
    match tokind {
        TokenKind::Add => "Add",
        TokenKind::As => "As",
        TokenKind::Audit => "Audit",
        TokenKind::Connect => "Connect",
        TokenKind::Create => "Create",
        TokenKind::Conn => "Conn",
        TokenKind::Contains => "Contains",
        TokenKind::Delete => "Delete",
        TokenKind::Drop => "Drop",
        TokenKind::Describe => "Describe",
        TokenKind::Destroy => "Destroy",
        TokenKind::Disable => "Disable",
        TokenKind::Disconnect => "Disconnect",
        TokenKind::Enable => "Enable",
        TokenKind::Entry => "Entry",
        TokenKind::From => "From",
        TokenKind::Generate => "Generate",
        TokenKind::Generated => "Generated",
        TokenKind::Init => "Init",
        TokenKind::Insert => "Insert",
        TokenKind::Into => "Into",
        TokenKind::List => "List",
        TokenKind::Limit => "Limit",
        TokenKind::EmptyIdentifer => "Identifier",
        TokenKind::Metadata => "Metadata",
        TokenKind::Minus => "Minus",
        TokenKind::Password => "Password",
        TokenKind::Percent => "Percent",
        TokenKind::Prompt => "Prompt",
        TokenKind::Plus => "Plus",
        TokenKind::Register => "Register",
        TokenKind::Rotate => "Rotate",
        TokenKind::Set => "Set",
        TokenKind::Select => "Select",
        TokenKind::Status => "Status",
        TokenKind::Slash => "Slash",
        TokenKind::Update => "Update",
        TokenKind::Where => "Where",
        TokenKind::With => "With",
        TokenKind::Bool(_) => "Bool",
        TokenKind::Identifier(_) => "Identifier",
        TokenKind::String(_) => "String",
        TokenKind::Number(_) => "Number",
        TokenKind::Semicolon => "Semicolon",
        TokenKind::Log => "Log",
        TokenKind::And => "And",
        TokenKind::Comma => "Comma",
        TokenKind::Or => "Or",
        TokenKind::Ge => "Ge",
        TokenKind::Gt => "Gt",
        TokenKind::Le => "Le",
        TokenKind::Lt => "Lt",
        TokenKind::To => "To",
        TokenKind::Equals => "Equals",
        TokenKind::NotEquals => "NotEquals",
        TokenKind::LeftParen => "LeftParen",
        TokenKind::RightParen => "RightParen",
        TokenKind::Astrisk => "Astrisk",
    }
}
