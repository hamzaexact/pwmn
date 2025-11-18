use std::fmt::{Display, Formatter, format, write};
const ERR_LEN: usize = 10;
pub struct Lexer<'a> {
    input: &'a str,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    pos: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Tokens {
    Add,
    As,
    Audit,
    Connect,
    Create,
    Contains,
    Delete,
    Drop,
    Describe,
    Destroy,
    Disable,
    Disconnect,
    Enable,
    From,
    Generate,
    Generated,
    Init,
    Insert,
    Into,
    List,
    Limit,
    Metadata,
    Password,
    Prompt,
    Register,
    Rotate,
    Set,
    Select,
    Status,
    Update,
    Where,
    With,

    Bool(bool),
    Identifer(String),
    String(String),
    Number(i32),

    Semicolon,
    Log,

    And,
    Comma,
    Or,
    Ge,
    Gt,
    Le,
    Lt,
    To,
    Equals,
    NotEquals,
    LeftParen,
    RightParen,
    Astrisk,
}

#[derive(Debug)]
pub enum LexerErr {
    InvalidNumber(String, usize),
    UnexpectedChar(String, usize, char),
    UnterminatedString(String, usize),
    UnterminatedParenthsis(String, usize),
    UnmatchedClosingParenthesis(String, usize),
}
use LexerErr::*;
use rustyline::completion::Quote;

impl Display for LexerErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidNumber(input, pos) => {
                write!(
                    f,
                    "{}",
                    err_formatter(
                        &format!("Invalid number at position {}", pos),
                        input,
                        *pos,
                        None,
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
    let pointer = format!("{}{}", " ".repeat(s_idx + 1), "^",);

    format!(
        "{}\n\t {}\n\t{}\nHint: {}\n",
        err_title, input, pointer, hint
    )
}



impl<'a> Lexer<'a> {
    pub fn tokenize(input: &str) -> Result<Vec<Tokens>, LexerErr> {
        let mut lexer = Lexer {
            input,
            chars: input.chars().peekable(),
            pos: 0,
        };

        match lexer.tokenize_input() {
            Ok(tokens) => Ok(tokens),
            Err(e) => Err(e),
        }
    }
    fn next_char(&mut self) -> Option<char> {
        let ch = self.chars.next()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }

    fn tokenize_input(&mut self) -> Result<Vec<Tokens>, LexerErr> {
        let mut tokens: Vec<Tokens> = Vec::new();
        let mut parenth_stack: Vec<usize> = Vec::new();
        let mut quotes_stack: Vec<usize> = Vec::new();

        while let Some(&char) = self.chars.peek() {
            match char {
                ' ' | '\t' | '\n' => {
                    self.next_char();
                }
                '"' | '\'' => {
                    quotes_stack.push(self.pos);
                    let quote = char;
                    self.next_char();
                    let mut string = String::new();
                    while let Some(&ch) = self.chars.peek() {
                        if quote == ch {
                            quotes_stack.pop();
                            self.next_char();
                            break;
                        }
                        string.push(ch);
                        self.next_char();
                    } // end of the inner while loop

                    if !quotes_stack.is_empty() {
                        return Err(LexerErr::UnterminatedString(
                            self.input.to_string(),
                            quotes_stack.pop().unwrap(),
                        ));
                    }

                    tokens.push(Tokens::String(string));
                } // end of string parse

                '-' | '0'..='9' => {
                    let mut signed = false;
                    if char == '-' {
                        signed = true;
                        self.next_char();
                    }
                    match self.extract_number(signed) {
                        Ok(tok) => tokens.push(tok),
                        Err(e) => return Err(e),
                    }
                }
                '=' => {
                    tokens.push(Tokens::Equals);
                    self.next_char();
                }
                '!' => {
                    self.next_char();
                    if let Some('=') = self.chars.peek() {
                        self.next_char();
                        tokens.push(Tokens::NotEquals);
                    } else {
                        return Err(LexerErr::UnexpectedChar(
                            self.input.to_string(),
                            self.pos,
                            char,
                        ));
                    }
                }

                ',' => {
                    self.next_char();
                    tokens.push(Tokens::Comma);
                }
                ';' => {
                    self.next_char();

                    tokens.push(Tokens::Semicolon);
                }

                '>' => {
                    self.next_char();
                    if let Some('=') = self.chars.peek() {
                        self.next_char();
                        tokens.push(Tokens::Ge);
                    } else {
                        tokens.push(Tokens::Gt);
                    }
                }
                '<' => {
                    self.next_char();
                    if let Some('=') = self.chars.next() {
                        self.next_char();
                        tokens.push(Tokens::Le);
                    } else {
                        tokens.push(Tokens::Lt);
                    }
                }
                ')' => {
                    if parenth_stack.is_empty() {
                        return Err(LexerErr::UnmatchedClosingParenthesis(
                            self.input.to_string(),
                            self.pos,
                        ));
                    }
                    parenth_stack.pop();
                    self.next_char();
                    tokens.push(Tokens::RightParen);
                }
                '(' => {
                    parenth_stack.push(self.pos);
                    self.next_char();
                    tokens.push(Tokens::LeftParen);
                }

                '*' => {
                    self.next_char();
                    tokens.push(Tokens::Astrisk);
                }

                _ if char.is_alphabetic() || char == '_' => {
                    let mut word = String::new();
                    while let Some(&ch) = self.chars.peek() {
                        if ch.is_alphabetic() || ch == '_' {
                            word.push(ch);
                            self.next_char();
                        } else {
                            break;
                        }
                    }
                    let token = match word.to_uppercase().as_str() {
                        "ADD" => Tokens::Add,
                        "AS" => Tokens::As,
                        "AUDIT" => Tokens::Audit,
                        "CONNECT" => Tokens::Connect,
                        "CREATE" => Tokens::Create,
                        "CONTAINS" => Tokens::Contains,
                        "DROP" => Tokens::Drop,
                        "DELETE" => Tokens::Delete,
                        "DESCRIBE" => Tokens::Describe,
                        "DESTROY" => Tokens::Destroy,
                        "DISABLE" => Tokens::Disable,
                        "DISCONNECT" => Tokens::Disconnect,
                        "ENABLE" => Tokens::Enable,
                        "FROM" => Tokens::From,
                        "GENERATE" => Tokens::Generate,
                        "GENERATED" => Tokens::Generated,
                        "INIT" => Tokens::Init,
                        "INTO" => Tokens::Into,
                        "INSERT" => Tokens::Insert,
                        "LIST" => Tokens::List,
                        "LOG" => Tokens::Log,
                        "LIMIT" => Tokens::Limit,
                        "METADATA" => Tokens::Metadata,
                        "PASSWORD" => Tokens::Password,
                        "PROMPT" => Tokens::Prompt,
                        "REGISTER" => Tokens::Register,
                        "ROTATE" => Tokens::Rotate,
                        "SELECT" => Tokens::Select,
                        "SET" => Tokens::Set,
                        "STATUS" => Tokens::Status,
                        "UPDATE" => Tokens::Update,
                        "WHERE" => Tokens::Where,
                        "WITH" => Tokens::With,
                        "TO" => Tokens::To,
                        "TRUE" => Tokens::Bool(true),
                        "FALSE" => Tokens::Bool(false),
                        "AND" => Tokens::And,
                        "OR" => Tokens::Or,
                        _ => Tokens::Identifer(word),
                    };
                    tokens.push(token);
                }
                _ => {
                    return Err(LexerErr::UnexpectedChar(
                        self.input.to_string(),
                        self.pos,
                        char,
                    ));
                }
            } // end of char matching
        } // end of the outer while loop

        if !parenth_stack.is_empty() {
            return Err(LexerErr::UnterminatedParenthsis(
                self.input.to_string(),
                parenth_stack.pop().unwrap(),
            ));
        }

        Ok(tokens)
    } // end of fn tokenize_input
    //

    fn extract_number(&mut self, signed: bool) -> Result<Tokens, LexerErr> {
        let mut str_number = if signed {
            String::from('-')
        } else {
            String::new()
        };
        // let mut chars = self.input.chars().peekable();
        while let Some(n) = self.chars.peek() {
            if n.is_digit(10) {
                str_number.push(*n);
                self.next_char();
            } else {
                break;
            }
        }
        let number = str_number
            .parse::<i32>()
            .map_err(|_| LexerErr::InvalidNumber);
        match number {
            Ok(n) => Ok(Tokens::Number(n)),
            Err(e) => Err(LexerErr::InvalidNumber(self.input.to_string(), self.pos)),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_simple_command() {
        let input = "CREATE REGISTER phone;";
        let tokens = Lexer::tokenize(input).unwrap();

        assert_eq!(
            tokens,
            vec![
                Tokens::Create,
                Tokens::Register,
                Tokens::Identifer("phone".to_string()),
                Tokens::Semicolon,
            ]
        );
    }

    #[test]
    fn test_string_literal() {
        let input = r#"ADD INTO phone PASSWORD "hunter2";"#;
        let tokens = Lexer::tokenize(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Tokens::Add, 
                Tokens::Into,
                Tokens::Identifer("phone".to_string()),
                Tokens::Password,
                Tokens::String("hunter2".to_string()),
                Tokens::Semicolon,
            ]
        );
    }

    #[test]
    fn test_numbers() {
        let input = "LIMIT 50";
        let tokens = Lexer::tokenize(input).unwrap();

        assert_eq!(tokens, vec![Tokens::Limit, Tokens::Number(50),]);
    }

    #[test]
    fn test_operators() {
        let input = "WHERE age >= 18 AND active = true";
        let tokens = Lexer::tokenize(input).unwrap();

        assert_eq!(
            tokens,
            vec![
                Tokens::Where,
                Tokens::Identifer("age".to_string()),
                Tokens::Ge,
                Tokens::Number(18),
                Tokens::And,
                Tokens::Identifer("active".to_string()),
                Tokens::Equals,
                Tokens::Bool(true),
            ]
        );
    }

    #[test]
    fn test_complex_query() {
        let input = "SELECT * FROM phone WHERE used_for CONTAINS \"github\";";
        let tokens = Lexer::tokenize(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Tokens::Select,
                Tokens::Astrisk,
                Tokens::From,
                Tokens::Identifer("phone".to_string()),
                Tokens::Where,
                Tokens::Identifer("used_for".to_string()),
                Tokens::Contains,
                Tokens::String("github".to_string()),
                Tokens::Semicolon,
            ]
        );
    }

    #[test]
    fn test_whitespace_handling() {
        let input = "CREATE    REGISTER     phone;"; 
        let tokens = Lexer::tokenize(input).unwrap();

        assert_eq!(
            tokens,
            vec![
                Tokens::Create,
                Tokens::Register,
                Tokens::Identifer("phone".to_string()),
                Tokens::Semicolon,
            ]
        );
    }

    #[test]
    fn test_case_insensitive() {
        let input = "create REGISTER Phone;";
        let tokens = Lexer::tokenize(input).unwrap();

        assert_eq!(
            tokens,
            vec![
                Tokens::Create,
                Tokens::Register,
                Tokens::Identifer("Phone".to_string()), // Identifiers keep original case
                Tokens::Semicolon,
            ]
        );
    }

    #[test]
    fn test_error_invalid_character() {
        let input = "CREATE REGISTER phone@;"; // @ is invalid
        let result = Lexer::tokenize(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let tokens = Lexer::tokenize(input).unwrap();

        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn test_unterminated_string() {
        let input = r#"PASSWORD "hunter2"#; // Missing closing quote
        let result = Lexer::tokenize(input);
        assert!(result.is_err());
    }
    #[test]
    fn test_negative_number() {
        let tokens = Lexer::tokenize("LIMIT -5").unwrap();
        assert_eq!(tokens, vec![Tokens::Limit, Tokens::Number(-5),]);
    }

    #[test]
    fn test_parentheses() {
        let tokens = Lexer::tokenize("WHERE (age > 18 AND active = true)").unwrap();
        assert!(tokens.contains(&Tokens::LeftParen));
        assert!(tokens.contains(&Tokens::RightParen));
    }

    #[test]
    fn test_unmatched_parentheses() {
        assert!(Lexer::tokenize("WHERE (age > 18").is_err());
        assert!(Lexer::tokenize("WHERE age > 18)").is_err());
    }

    #[test]
    fn test_all_comparison_operators() {
        let tokens = Lexer::tokenize("> >= < <= = !=").unwrap();
        assert_eq!(
            tokens,
            vec![
                Tokens::Gt,
                Tokens::Ge,
                Tokens::Lt,
                Tokens::Le,
                Tokens::Equals,
                Tokens::NotEquals,
            ]
        );
    }
}
