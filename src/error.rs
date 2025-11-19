use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum LexerErr {
    InvalidNumber(String, usize, usize),
    UnexpectedChar(String, usize, char),
    UnterminatedString(String, usize),
    UnterminatedParenthsis(String, usize),
    UnmatchedClosingParenthesis(String, usize),
}
use LexerErr::*;

impl Display for LexerErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidNumber(input, s_idx, e_idx) => {
                write!(
                    f,
                    "{}",
                    err_formatter(
                        &format!("Invalid number from position {} to {}", s_idx, e_idx),
                        input,
                        *s_idx,
                        Some(e_idx),
                        "Number too large or malformed"
                    )
                )
            }
            Self::UnexpectedChar(input, s_idx, char) => {
                let err_title = format!("Unexpected Character '{}' at position {}", char, s_idx);
                let hint = "Expected alphanumeric, operator, or keyword";
                write!(
                    f,
                    "{}",
                    err_formatter(err_title.as_str(), input, *s_idx, None, hint)
                )
            }
            Self::UnterminatedString(input, idx) => {
                write!(
                    f,
                    "{}",
                    err_formatter(
                        &format!("Unterminated string at position {}", idx),
                        input,
                        *idx,
                        None,
                        "Expected closing quote"
                    )
                )
            }

            Self::UnterminatedParenthsis(input, idx) => {
                write!(
                    f,
                    "{}",
                    err_formatter(
                        &format!("Unclosed parenthesis at position {}", idx),
                        input,
                        *idx,
                        None,
                        "Expected ')'"
                    )
                )
            }

            Self::UnmatchedClosingParenthesis(input, idx) => {
                write!(
                    f,
                    "{}",
                    err_formatter(
                        &format!("Unmatched ')' at position {}", idx),
                        input,
                        *idx,
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
