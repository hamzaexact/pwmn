use std::fmt::{format, write, Display, Formatter};
const ERR_LEN:usize = 10;
pub struct Lexer<'a> {
    input: &'a str,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    pos: usize,
}

#[derive(Debug)]
pub enum Tokens {
    Create,
    Register,
    Init,
    Update,
    Fetch,
    Describe,
    Into,
    Add,
    List,
    Set,
    Delete,
    Enable,
    Disable,
    Destroy,
    Rotate,
    Audit,
    Disconnect,
    Connect,
    From,
    Where,

    Identifer(String),
    String(String),
    Number(i32),
    Bool(bool),

    Semicolon,
    As,
    Log,
    Equals,

    NotEquals,
    LeftParen,
    RightParen,
    And,
    Or,
    Contains,
    With,
}

#[derive(Debug)]
pub enum LexerErr {
    EmptyInput,
    UnexpectedEndOfInput,
    // InvalidSyntax,
    UnexpectedChar(String, usize, Option<usize>),
    InvalidNumber,
    ExpectedEndOfInput(String, usize, Option<usize>)
}
use LexerErr::*;

impl Display for LexerErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "Expected Query, BUT FOUND Nothing"),
            Self::InvalidNumber => write!(f, "Could not parse"),
            Self::UnexpectedChar(input, s_idx, e_idx) => {
                let err_msg = "Unexpected Character near index";
                write!(f, "{}", lexer_err(input, s_idx, e_idx, err_msg))
            }
            Self::ExpectedEndOfInput(input, s_idx, e_idx) => write!(f,"{}",lexer_err(input, s_idx, e_idx, "Expected End of Input, Syntax Err near to index")),
            _ => todo!(),
        }
    }
}

fn lexer_err(input:&str, s_idx: &usize, e_idx: &Option<usize>, err_msg: &str) -> String {
    let mut err_vec = vec!["-"; *s_idx+1];
    let  _:Vec<_> = err_vec.iter_mut()
    .enumerate()
    .map(|(index, char)| if index == *s_idx {
        *char = "^"})
    .collect();
    let str:String = err_vec.into_iter().collect();
    format!("{} {}:\n\t\t{}\t\n{}{}",  err_msg, s_idx, input, " ".repeat(16), str)
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
        // println!("pos: {:?}", self.pos);
        // println!("char: {:?}", ch);
        Some(ch)
    }

    fn tokenize_input(&mut self) -> Result<Vec<Tokens>, LexerErr> {
        let mut tokens: Vec<Tokens> = Vec::new();
        if self.chars.peek().is_none() {
            return Err(LexerErr::EmptyInput);
        }
        // let mut chars = self.input.chars().peekable();
        while let Some(&char) = self.chars.peek() {
            match char {
                ' ' | '\t' | '\n' => {
                    self.next_char();
                }
                '"' | '\'' => {
                    let quote = char;
                    self.next_char();
                    let mut string = String::new();
                    while let Some(&ch) = self.chars.peek() {
                        if quote == ch {
                            self.next_char();
                            break;
                        }
                        string.push(ch);
                        self.next_char();
                    } // end of the inner while loop
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
                        return  Err(LexerErr::UnexpectedChar(self.input.to_string(), self.pos-1, None));
                    }
                }
                ';' => {
                    self.next_char();
                    if self.chars.peek().is_some() {
                        return Err(LexerErr::ExpectedEndOfInput(self.input.to_string(), self.pos, None));
                    }
                    tokens.push(Tokens::Semicolon);
                }
                _ => unreachable!(),
            } // end of char matching
        } // end of the outer while loop
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
            Err(e) => Err(LexerErr::InvalidNumber),
        }
    }
}

//
// pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
//     let mut tokens = Vec::new();
//     let mut chars = input.chars().peekable();
//     // let k = chars.peek();
//     while let Some(&ch) = chars.peek() {
//         match ch {
//             '\n' | '\t' | '\n' => {
//                 chars.next();
//             }
//
//             '"' | '\'' => {
//                 let quote = ch;
//                 let mut string = String::new();
//                 chars.next();
//                 while let Some(&char) = chars.peek() {
//                     if char == quote {
//                         chars.next();
//                         break;
//                     }
//                     string.push(char);
//                     chars.next();
//                 }
//                 tokens.push(Token::String(string));
//                 return Ok(tokens);
//             }
//             '-' => {
//                 let mut number = String::new();
//                 chars.next();
//                 while let Some(&char) = chars.peek() {
//                     if char.is
//                 }
//             }
//         }
//     }
//     Err(String::new())
// }
