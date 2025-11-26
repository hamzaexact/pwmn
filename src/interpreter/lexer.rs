use crate::error::LexerErr;
use crate::interpreter::push_token;
use std::fmt::{Display, Formatter, format, write};
const ERR_LEN: usize = 10;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    pos: usize,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct LexResult<'a> {
    pub query: &'a str,
    pub tokens: Vec<Token>,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum TokenKind {
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
    Minus,
    Password,
    Percent,
    Slash,
    Plus,
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
    Identifier(String),
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

impl<'a> Lexer<'a> {
    pub fn tokenize(input: &str) -> Result<LexResult, LexerErr> {
        let mut lexer = Lexer {
            input,
            chars: input.chars().peekable(),
            pos: 0,
        };

        match lexer.tokenize_input() {
            Ok(tokens) => Ok(LexResult {
                query: input,
                tokens: tokens,
            }),
            Err(e) => Err(e),
        }
    }
    fn next_char(&mut self) -> Option<char> {
        let ch = self.chars.next()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }

    fn tokenize_input(&mut self) -> Result<Vec<Token>, LexerErr> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut parenth_stack: Vec<usize> = Vec::new();
        let mut quotes_stack: Vec<usize> = Vec::new();
        let mut start: usize = self.pos;

        while let Some(&char) = self.chars.peek() {
            match char {
                ' ' | '\t' | '\n' => {
                    self.next_char();
                }
                '"' | '\'' => {
                    start = self.pos;
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
                            Span {
                                start: quotes_stack.pop().unwrap(),
                                end: self.pos,
                            },
                        ));
                    }

                    push_token(&mut tokens, TokenKind::String(string), start, self.pos);
                } // end of string parse

                '-' => {
                    start = self.pos;
                    self.next_char();
                    push_token(&mut tokens, TokenKind::Minus, start, self.pos);
                }

                '0'..='9' => {
                    start = self.pos;
                    match self.extract_number() {
                        Ok(tok) => push_token(&mut tokens, tok, start, self.pos),
                        Err(e) => return Err(e),
                    }
                }
                '=' => {
                    start = self.pos;
                    self.next_char();
                    push_token(&mut tokens, TokenKind::Equals, start, self.pos);
                }
                '!' => {
                    start = self.pos;
                    self.next_char();
                    if let Some('=') = self.chars.peek() {
                        self.next_char();
                        push_token(&mut tokens, TokenKind::NotEquals, start, self.pos);
                    } else {
                        return Err(LexerErr::UnexpectedChar(
                            self.input.to_string(),
                            char,
                            Span {
                                start: start,
                                end: self.pos,
                            },
                        ));
                    }
                }

                ',' => {
                    start = self.pos;
                    self.next_char();
                    push_token(&mut tokens, TokenKind::Comma, start, self.pos);
                }
                ';' => {
                    start = self.pos;
                    self.next_char();
                    push_token(&mut tokens, TokenKind::Semicolon, start, self.pos);
                }

                '>' => {
                    start = self.pos;
                    self.next_char();
                    if let Some('=') = self.chars.peek() {
                        self.next_char();
                        push_token(&mut tokens, TokenKind::Ge, start, self.pos);
                    } else {
                        push_token(&mut tokens, TokenKind::Gt, start, self.pos);
                    }
                }
                '<' => {
                    start = self.pos;
                    self.next_char();
                    if let Some('=') = self.chars.next() {
                        self.next_char();
                        push_token(&mut tokens, TokenKind::Le, start, self.pos);
                    } else {
                        push_token(&mut tokens, TokenKind::Lt, start, self.pos);
                    }
                }

                '+' => {
                    start = self.pos;
                    self.next_char();
                    push_token(&mut tokens, TokenKind::Plus, start, self.pos);
                }

                ')' => {
                    start = self.pos;
                    if parenth_stack.is_empty() {
                        return Err(LexerErr::UnmatchedClosingParenthesis(
                            self.input.to_string(),
                            Span {
                                start: start,
                                end: self.pos,
                            },
                        ));
                    }
                    parenth_stack.pop();
                    self.next_char();
                    push_token(&mut tokens, TokenKind::RightParen, start, self.pos);
                }
                '(' => {
                    start = self.pos;
                    parenth_stack.push(self.pos);
                    self.next_char();
                    push_token(&mut tokens, TokenKind::LeftParen, start, self.pos);
                }

                '*' => {
                    start = self.pos;
                    self.next_char();
                    push_token(&mut tokens, TokenKind::Astrisk, start, self.pos);
                }

                _ if char.is_alphabetic() || char == '_' => {
                    let start = self.pos;
                    let mut word = String::new();

                    while let Some(&ch) = self.chars.peek() {
                        if ch.is_alphabetic() || ch.is_digit(10) || ch == '_' {
                            word.push(ch);
                            self.next_char();
                        } else {
                            break;
                        }
                    }

                    let upper = word.to_uppercase();

                    let kind = match upper.as_str() {
                        "ADD" => TokenKind::Add,
                        "AS" => TokenKind::As,
                        "AUDIT" => TokenKind::Audit,
                        "CONNECT" => TokenKind::Connect,
                        "CREATE" => TokenKind::Create,
                        "CONTAINS" => TokenKind::Contains,
                        "DROP" => TokenKind::Drop,
                        "DELETE" => TokenKind::Delete,
                        "DESCRIBE" => TokenKind::Describe,
                        "DESTROY" => TokenKind::Destroy,
                        "DISABLE" => TokenKind::Disable,
                        "DISCONNECT" => TokenKind::Disconnect,
                        "ENABLE" => TokenKind::Enable,
                        "FROM" => TokenKind::From,
                        "GENERATE" => TokenKind::Generate,
                        "GENERATED" => TokenKind::Generated,
                        "INIT" => TokenKind::Init,
                        "INTO" => TokenKind::Into,
                        "INSERT" => TokenKind::Insert,
                        "LIST" => TokenKind::List,
                        "LOG" => TokenKind::Log,
                        "LIMIT" => TokenKind::Limit,
                        "METADATA" => TokenKind::Metadata,
                        "PASSWORD" => TokenKind::Password,
                        "PROMPT" => TokenKind::Prompt,
                        "REGISTER" => TokenKind::Register,
                        "REG" => TokenKind::Register, // shorthand for register;
                        "ROTATE" => TokenKind::Rotate,
                        "SELECT" => TokenKind::Select,
                        "SET" => TokenKind::Set,
                        "STATUS" => TokenKind::Status,
                        "UPDATE" => TokenKind::Update,
                        "WHERE" => TokenKind::Where,
                        "WITH" => TokenKind::With,
                        "TO" => TokenKind::To,
                        "AND" => TokenKind::And,
                        "OR" => TokenKind::Or,
                        "TRUE" => TokenKind::Bool(true),
                        "FALSE" => TokenKind::Bool(false),
                        _ => TokenKind::Identifier(word.to_string()),
                    };

                    push_token(&mut tokens, kind, start, self.pos);
                }
                _ => {
                    return Err(LexerErr::UnexpectedChar(
                        self.input.to_string(),
                        char,
                        Span {
                            start: self.pos,
                            end: self.pos + 1,
                        },
                    ));
                }
            } // end of char matching
        } // end of the outer while loop

        if !parenth_stack.is_empty() {
            return Err(LexerErr::UnterminatedParenthsis(
                self.input.to_string(),
                Span {
                    start: parenth_stack.pop().unwrap(),
                    end: self.pos,
                },
            ));
        }

        Ok(tokens)
    } // end of fn tokenize_input
    //

    fn extract_number(&mut self) -> Result<TokenKind, LexerErr> {
        let mut s_idx: usize = self.pos;
        let mut str_number = String::new();
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
            .map_err(|_| LexerErr::InvalidNumber {
                input: self.input.to_string(),
                span: Span {
                    start: s_idx,
                    end: self.pos,
                },
            });
        match number {
            Ok(n) => Ok(TokenKind::Number(n)),
            Err(e) => Err(LexerErr::InvalidNumber {
                input: self.input.to_string(),
                span: Span {
                    start: s_idx,
                    end: self.pos,
                },
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create token with span
    fn tok(kind: TokenKind, start: usize, end: usize) -> Token {
        Token {
            kind,
            span: Span { start, end },
        }
    }

    #[test]
    fn test_simple_command() {
        let input = "CREATE REGISTER phone;";
        let tokens = Lexer::tokenize(input).unwrap().tokens;

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].kind, TokenKind::Create);
        assert_eq!(tokens[1].kind, TokenKind::Register);
        assert_eq!(tokens[2].kind, TokenKind::Identifier("phone".to_string()));
        assert_eq!(tokens[3].kind, TokenKind::Semicolon);
    }

    #[test]
    fn test_string_literal() {
        let input = r#"ADD INTO phone PASSWORD "hunter2";"#;
        let tokens = Lexer::tokenize(input).unwrap().tokens;

        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0].kind, TokenKind::Add);
        assert_eq!(tokens[1].kind, TokenKind::Into);
        assert_eq!(tokens[2].kind, TokenKind::Identifier("phone".to_string()));
        assert_eq!(tokens[3].kind, TokenKind::Password);
        assert_eq!(tokens[4].kind, TokenKind::String("hunter2".to_string()));
        assert_eq!(tokens[5].kind, TokenKind::Semicolon);
    }

    #[test]
    fn test_numbers() {
        let input = "LIMIT 50";
        let tokens = Lexer::tokenize(input).unwrap().tokens;

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::Limit);
        assert_eq!(tokens[1].kind, TokenKind::Number(50));
    }

    #[test]
    fn test_negative_number() {
        let input = "LIMIT -5";
        let tokens = Lexer::tokenize(input).unwrap().tokens;

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::Limit);
        assert_eq!(tokens[1].kind, TokenKind::Number(-5));
    }

    #[test]
    fn test_operators() {
        let input = "WHERE age >= 18 AND active = true";
        let tokens = Lexer::tokenize(input).unwrap().tokens;

        assert_eq!(tokens.len(), 8);
        assert_eq!(tokens[0].kind, TokenKind::Where);
        assert_eq!(tokens[1].kind, TokenKind::Identifier("age".to_string()));
        assert_eq!(tokens[2].kind, TokenKind::Ge);
        assert_eq!(tokens[3].kind, TokenKind::Number(18));
        assert_eq!(tokens[4].kind, TokenKind::And);
        assert_eq!(tokens[5].kind, TokenKind::Identifier("active".to_string()));
        assert_eq!(tokens[6].kind, TokenKind::Equals);
        assert_eq!(tokens[7].kind, TokenKind::Bool(true));
    }

    #[test]
    fn test_all_comparison_operators() {
        let input = "> >= < <= = !=";
        let tokens = Lexer::tokenize(input).unwrap().tokens;

        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0].kind, TokenKind::Gt);
        assert_eq!(tokens[1].kind, TokenKind::Ge);
        assert_eq!(tokens[2].kind, TokenKind::Lt);
        assert_eq!(tokens[3].kind, TokenKind::Le);
        assert_eq!(tokens[4].kind, TokenKind::Equals);
        assert_eq!(tokens[5].kind, TokenKind::NotEquals);
    }

    #[test]
    fn test_complex_query() {
        let input = "SELECT * FROM phone WHERE used_for CONTAINS \"github\";";
        let tokens = Lexer::tokenize(input).unwrap().tokens;

        assert_eq!(tokens.len(), 9);
        assert_eq!(tokens[0].kind, TokenKind::Select);
        assert_eq!(tokens[1].kind, TokenKind::Astrisk);
        assert_eq!(tokens[2].kind, TokenKind::From);
        assert_eq!(tokens[3].kind, TokenKind::Identifier("phone".to_string()));
        assert_eq!(tokens[4].kind, TokenKind::Where);
        assert_eq!(
            tokens[5].kind,
            TokenKind::Identifier("used_for".to_string())
        );
        assert_eq!(tokens[6].kind, TokenKind::Contains);
        assert_eq!(tokens[7].kind, TokenKind::String("github".to_string()));
        assert_eq!(tokens[8].kind, TokenKind::Semicolon);
    }

    #[test]
    fn test_whitespace_handling() {
        let input = "CREATE    REGISTER     phone;";
        let tokens = Lexer::tokenize(input).unwrap().tokens;

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].kind, TokenKind::Create);
        assert_eq!(tokens[1].kind, TokenKind::Register);
        assert_eq!(tokens[2].kind, TokenKind::Identifier("phone".to_string()));
        assert_eq!(tokens[3].kind, TokenKind::Semicolon);
    }

    #[test]
    fn test_case_insensitive_keywords() {
        let input = "create REGISTER Phone;";
        let tokens = Lexer::tokenize(input).unwrap().tokens;

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].kind, TokenKind::Create);
        assert_eq!(tokens[1].kind, TokenKind::Register);
        assert_eq!(tokens[2].kind, TokenKind::Identifier("Phone".to_string()));
        assert_eq!(tokens[3].kind, TokenKind::Semicolon);
    }

    #[test]
    fn test_parentheses() {
        let input = "WHERE (age > 18 AND active = true)";
        let tokens = Lexer::tokenize(input).unwrap().tokens;

        assert!(tokens.iter().any(|t| t.kind == TokenKind::LeftParen));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::RightParen));
    }

    #[test]
    fn test_comma() {
        let input = "SELECT id, name, password FROM phone;";
        let tokens = Lexer::tokenize(input).unwrap().tokens;

        assert!(tokens.iter().any(|t| t.kind == TokenKind::Comma));
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let tokens = Lexer::tokenize(input).unwrap().tokens;
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_error_invalid_character() {
        let input = "CREATE REGISTER phone@;";
        let result = Lexer::tokenize(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_error_unterminated_string() {
        let input = r#"PASSWORD "hunter2"#;
        let result = Lexer::tokenize(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_error_unmatched_opening_paren() {
        let input = "WHERE (age > 18";
        let result = Lexer::tokenize(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_error_unmatched_closing_paren() {
        let input = "WHERE age > 18)";
        let result = Lexer::tokenize(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_span_tracking() {
        let input = "CREATE REGISTER phone;";
        let tokens = Lexer::tokenize(input).unwrap().tokens;

        // CREATE at position 0-6
        assert_eq!(tokens[0].span.start, 0);
        assert_eq!(tokens[0].span.end, 6);

        // REGISTER at position 7-15
        assert_eq!(tokens[1].span.start, 7);
        assert_eq!(tokens[1].span.end, 15);
    }

    #[test]
    fn test_underscore_in_identifier() {
        let input = "my_register_name";
        let tokens = Lexer::tokenize(input).unwrap().tokens;

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens[0].kind,
            TokenKind::Identifier("my_register_name".to_string())
        );
    }

    #[test]
    fn test_boolean_literals() {
        let input = "true AND false";
        let tokens = Lexer::tokenize(input).unwrap().tokens;

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].kind, TokenKind::Bool(true));
        assert_eq!(tokens[1].kind, TokenKind::And);
        assert_eq!(tokens[2].kind, TokenKind::Bool(false));
    }

    #[test]
    fn test_complex_nested_query() {
        let input =
            "WHERE (used_for CONTAINS \"aws\" OR used_for CONTAINS \"s3\") AND active = true;";
        let result = Lexer::tokenize(input);
        assert!(result.is_ok());
        let tokens = result.unwrap().tokens;

        // Verify parentheses are balanced
        let open_count = tokens
            .iter()
            .filter(|t| t.kind == TokenKind::LeftParen)
            .count();
        let close_count = tokens
            .iter()
            .filter(|t| t.kind == TokenKind::RightParen)
            .count();
        assert_eq!(open_count, close_count);
    }
}
