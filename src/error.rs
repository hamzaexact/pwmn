use crate::interpreter::token_name;
use crate::interpreter::{Span, lexer::TokenKind};
use std::any::type_name;
use std::fmt::{Debug, Display, Formatter, format};
use std::fs::write;

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

#[derive(Debug)]
pub enum CreateErr {
    VaultNotExists,
    ValidationErr,
    RegisterAlreadyExists,
    DestroyedVaultErr,
    ShortLenErr { temp: String, target_len: u8 },
}

#[derive(Debug)]
pub enum SessionErr {
    SessionNotConnected,
    PermissionDenied,
    AnotherSessionIsRunningErr,
}

#[derive(Debug)]
pub enum ConnectionErr {
    VaultInvalidConnection(String),
}

#[derive(Debug)]
pub enum EncryptionErr {
    EncryptionErr,
}

#[derive(Debug)]
pub enum DecryptionErr {
    DecryptionErr,
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
impl std::error::Error for CreateErr {}
impl std::error::Error for SessionErr {}
impl std::error::Error for EncryptionErr {}
impl std::error::Error for DecryptionErr {}
impl std::error::Error for ConnectionErr {}

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

impl std::fmt::Display for CreateErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VaultNotExists => {
                write!(
                    f,
                    "There is no RootVault yet, run INIT first to initialize the RootVAULT"
                )
            }

            Self::ValidationErr => {
                write!(f, "Special characters are not allowed in Register names")
            }

            Self::RegisterAlreadyExists => {
                write!(
                    f,
                    "A register with this name already in the vault, use another name"
                )
            }
            Self::DestroyedVaultErr => {
                // todo, repair files logic
                write!(
                    f,
                    "The root vault file is missed or desroyed:\n\n  ROOT VAULT\n\t|\n\t| ->rvault.bin (missed)\n\n\tRun REPAIR VAULT to repair the desroyed files(TODO)"
                )
            }

            Self::ShortLenErr { temp, target_len } => {
                let err_msg = format!(
                    "Minimum Length for your {} must be at least {} characters",
                    *temp, *target_len
                );
                write!(f, "{err_msg}")
            }
        }
    }
}

impl std::fmt::Display for ConnectionErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VaultInvalidConnection(ConnFrom) => {
                let err_title = format!(
                    "You request a connection to a vault that does not exist\nDouble check your Register name or use CREATE REGISTER <{}>.",
                    ConnFrom
                );
                write!(f, "{err_title}")
            }
        }
    }
}

impl std::fmt::Display for SessionErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SessionNotConnected => {
                let err_title = "Not connected to any register. Use CONNECT <name>";
                write!(f, "{err_title}")
            }

            Self::AnotherSessionIsRunningErr => {
                let err_title = "To CREATE or CONNECT to a register, you must first disconnect the currently connected register";
                write!(f, "{err_title}")
            }

            _ => todo!(),
        }
    }
}
impl std::fmt::Display for EncryptionErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Couldn't encrypt the given data; something went wrong.")
    }
}

impl std::fmt::Display for DecryptionErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid password or corrupted vault data. Try Again!")
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
