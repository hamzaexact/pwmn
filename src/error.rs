use std::fmt::{Debug, Display, Formatter};

use crate::interpreter::Span;

#[derive(Debug)]
pub enum LexerErr {
    InvalidNumber(String, Span),
    UnexpectedChar(String, char, Span),
    UnterminatedString(String, Span),
    UnterminatedParenthsis(String, Span),
    UnmatchedClosingParenthesis(String, Span)
}
use LexerErr::*;

impl Display for LexerErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidNumber(input,  span)=> {
                write!(
                    f,
                    "{}",
                    err_formatter(
                        &format!("Invalid number from position {} to {}", span.start, span.end),
                        input,
                        span.start,
                        Some(&span.end),
                        "Number too large or malformed"
                    )
                )
            }
            Self::UnexpectedChar(input, char, span) => {
                let err_title = format!("Unexpected Character '{}' at position {}", char, span.start);
                let hint = "Expected alphanumeric, operator, or keyword";
                write!(
                    f,
                    "{}",
                    err_formatter(err_title.as_str(), input, span.start, None, hint)
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
                        "Expected closing quote"
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
                        "Expected ')'"
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
                        "No matching '(' found"
                    )
                )
            }

            _ => todo!(),
        }
    }
}

fn err_formatter(
    err_title: &str,
    input: &str,
    s_idx: usize,
    e_idx: Option<&usize>,
    hint: &str,
) -> String {
    let pointer = format!(
        "{}{}",
        " ".repeat(s_idx + 1),
        "^".repeat(if e_idx.is_some() {
            *e_idx.unwrap() - s_idx
        } else {
            1
        }),
    );

    format!(
        "{}\n\t {}\n\t{}\nHint: {}\n",
        err_title, input, pointer, hint
    )
}
