use crate::interpreter::token_name;
use crate::interpreter::{Span, lexer::TokenKind};
use std::any::type_name;
use std::fmt::{Debug, Display, Formatter, format};

#[derive(Debug)]
pub enum LexerErr {
    InvalidNumber { input: String, span: Span },
    UnexpectedChar(String, char, Span),
    UnterminatedString(String, Span),
    UnterminatedParenthsis(String, Span),
    UnmatchedClosingParenthesis(String, Span),
}

#[derive(Debug, Clone)]
pub enum ParserErr {
    UnexpectedEndOfExpression {
        input: String,
        tokind: TokenKind,
        span: Span,
    },
    TypeMismatch {
        input: String,
        expectedkind: TokenKind,
        givenkind: TokenKind,
        span: Span,
    },

    ExpectedIdentifier {
        input: String,
        givenkind: TokenKind,
        span: Span,
    },
}

#[derive(Debug)]
pub enum InitErr {
    VaultAlreadyExists,
}

pub enum ParserToken {
    Expression,
    Identifier,
    Keyword,
}

use LexerErr::*;
use ParserErr::*;
use ParserToken::*;

impl std::error::Error for LexerErr {}
impl std::error::Error for ParserErr {}
impl std::error::Error for InitErr {}

impl<'a> Display for LexerErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidNumber { input, span } => {
                write!(
                    f,
                    "{}",
                    err_formatter(
                        &format!(
                            "Invalid number from position {} to {}",
                            span.start, span.end
                        ),
                        input,
                        span.start,
                        Some(&span.end),
                        Some("Number too large or malformed")
                    )
                )
            }
            Self::UnexpectedChar(input, char, span) => {
                let err_title =
                    format!("Unexpected Character '{}' at position {}", char, span.start);
                let hint = "Expected alphanumeric, operator, or keyword";
                write!(
                    f,
                    "{}",
                    err_formatter(err_title.as_str(), input, span.start, None, Some(hint))
                )
            }
            Self::UnterminatedString(input, span) => {
                write!(
                    f,
                    "{}",
                    err_formatter(
                        &format!("Unterminated string at position {}", span.start),
                        input,
                        span.start,
                        None,
                        Some("Expected closing quote")
                    )
                )
            }

            Self::UnterminatedParenthsis(input, span) => {
                write!(
                    f,
                    "{}",
                    err_formatter(
                        &format!("Unclosed parenthesis at position {}", span.start),
                        input,
                        span.start,
                        None,
                        Some("Expected ')'")
                    )
                )
            }

            Self::UnmatchedClosingParenthesis(input, span) => {
                write!(
                    f,
                    "{}",
                    err_formatter(
                        &format!("Unmatched ')' at position {}", span.start),
                        input,
                        span.start,
                        None,
                        Some("No matching '(' found")
                    )
                )
            }
        }
    }
}

impl<'a> Display for ParserErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedEndOfExpression {
                input,
                tokind,
                span,
            } => {
                let err_msg = format!(
                    "Unexpected end of expression, expected '{}'",
                    token_name(tokind)
                );
                write!(
                    f,
                    "{}",
                    err_formatter(
                        err_msg.as_str(),
                        input.as_str(),
                        span.start,
                        Some(&span.end),
                        None
                    )
                )
            }
            Self::TypeMismatch {
                input,
                expectedkind,
                givenkind,
                span,
            } => {
                let err_title = format!(
                    "Expected Token type {}, got {}",
                    token_name(expectedkind),
                    token_name(givenkind)
                );
                write!(
                    f,
                    "{}",
                    err_formatter(err_title.as_str(), input, span.start, Some(&span.end), None)
                )
            }

            Self::ExpectedIdentifier {
                input,
                givenkind,
                span,
            } => {
                let err_title = format!(
                    "Expected identifier but {} token type were given",
                    token_name(givenkind)
                );

                write!(
                    f,
                    "{}",
                    err_formatter(err_title.as_str(), input, span.start, Some(&span.end), None)
                )
            }
        }
    }
}

impl std::fmt::Display for InitErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VaultAlreadyExists => {
                let err_title = "The Root Vault already exists, and it cannot be overided, TODO";
                write!(f, "{}", err_title)
            }
        }
    }
}

fn err_formatter(
    err_title: &str,
    input: &str,
    start: usize,
    end: Option<&usize>,
    hint: Option<&str>,
) -> String {
    let pointer = format!(
        "{}{}",
        " ".repeat(start + 1),
        "^".repeat(if end.is_some() {
            *end.unwrap() - start
        } else {
            1
        }),
    );

    let without_hint = format!("{}\n\t {}\n\t{}\n", err_title, input, pointer);

    if hint.is_some() {
        return format!(
            "{}\n\t {}\n\t{}\nHint: {}\n",
            err_title,
            input,
            pointer,
            hint.unwrap()
        );
    }
    without_hint
}
