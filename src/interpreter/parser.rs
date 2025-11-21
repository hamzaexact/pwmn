use crate::error::ParserErr;
use crate::interpreter::lexer::{LexResult, Span, Token, TokenKind};
use crate::interpreter::{ast, lexer};
use std::error::Error;
//
pub struct Parser<'t> {
    query: &'t str,
    tokens: Vec<Token>,
    pos: usize,
}

impl<'t> Parser<'t> {
    pub fn parse(lex_res: LexResult) -> Result<ast::Stmt, ParserErr> {
        let mut parser = Parser {
            query: lex_res.query,
            tokens: lex_res.tokens,
            pos: 0,
        };
        let f = parser.parse_expression()?;
        Ok(ast::Stmt::Empty)
    }

    fn peek_token(&self) -> Option<&Token> {
        if self.pos < self.tokens.len() {
            return Some(&self.tokens[self.pos]);
        }
        None
    }

    fn consume(&mut self, expected_token: TokenKind) -> Result<(), ParserErr> {
        let token = self.peek_token();
        match token {
            Some(tk) => {
                if (*tk).kind == expected_token {
                    self.pos += 1;
                } else {
                    return Err(ParserErr::TypeMismatch {
                        input: self.query.to_string(),
                        expectedkind: expected_token,
                        givenkind: tk.kind.clone(),
                        span: token.unwrap().span.clone(),
                    });
                }
            }
            None => {
                return Err(ParserErr::UnexpectedEndOfExpression {
                    input: self.query.to_string(),
                    tokind: expected_token,
                    span: Span {
                        start: self.pos,
                        end: self.pos + 1,
                    },
                });
            }
        }
        Ok(())
    }

    fn parse_expression(&mut self) -> Result<ast::Expr, ParserErr> {
        let expr = self.parse_addition()?;
        Ok(expr)
    }

    fn parse_addition(&mut self) -> Result<ast::Expr, ParserErr> {
        let expr = self.parse_multiplication()?;
        let token = self.peek_token();
        match token {
            Some(tk) => {
                if tk.kind == TokenKind::Plus {
                    self.consume(TokenKind::Plus);
                    let right = self.parse_multiplication()?;
                    return Ok(ast::Expr::Add(Box::new(expr), Box::new(right)));
                }
            }
            None => {
                println!("ERR");
            }
        }

        Ok(expr)
    }

    fn parse_multiplication(&mut self) -> Result<ast::Expr, ParserErr> {
        let expr = self.parse_factor()?;
        let token = self.peek_token();
        match token {
            Some(tk) => {
                if tk.kind == TokenKind::Astrisk {
                    let right = self.parse_multiplication()?;
                    return Ok(ast::Expr::Add(Box::new(expr), Box::new(right)));
                }
            }
            None => {
                println!("ERR");
            }
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<ast::Expr, ParserErr> {
        let token = self.peek_token();
        match token {
            Some(tk) => match tk.kind {
                TokenKind::Select => {
                    self.consume(TokenKind::Select);
                    let expr = self.parse_expression()?;
                    return Ok(ast::Expr::Statment(ast::Stmt::Select { cols: Box::new(expr)}));
                }

                TokenKind::Number(n) => {
                    self.consume(TokenKind::Number(n));
                    return Ok(ast::Expr::Number(n));
                }
                _ => return Ok(ast::Expr::Empty),
            },
            None => return Ok(ast::Expr::Empty),
        }
        Ok(ast::Expr::Empty)
    }
}
