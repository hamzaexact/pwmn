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
        let res = parser.parse_expression()?;
        match res {
            ast::Expr::Statment(ast::Stmt::Init) => Ok(ast::Stmt::Init),
            _ => todo!(),
        }
    }

    fn peek_token(&self) -> Option<(Token, TokenKind)> {
        if self.pos < self.tokens.len() {
            return Some((
                self.tokens[self.pos].clone(),
                self.tokens[self.pos].kind.clone(),
            ));
        }
        None
    }

    fn consume(&mut self, expected_token: TokenKind) -> Result<Token, ParserErr> {
        let token = self.peek_token();

        match token {
            Some(ref token) => {
                let (whole_token, token_kind) = token;
                if *token_kind == expected_token {
                    self.pos += 1;
                } else {
                    return Err(ParserErr::TypeMismatch {
                        input: self.query.to_string(),
                        expectedkind: expected_token,
                        givenkind: token_kind.clone(),
                        span: whole_token.span.clone(),
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
        return Ok(token.unwrap().0);
    }

    fn parse_expression(&mut self) -> Result<ast::Expr, ParserErr> {
        let expr = self.parse_addition()?;
        // dbg!(&expr);
        Ok(expr)
    }

    fn parse_addition(&mut self) -> Result<ast::Expr, ParserErr> {
        let expr = self.parse_multiplication()?;
        let token = self.peek_token();
        match token {
            Some(token) => {
                let (tk, kind) = token;
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
                let (tk, kind) = tk;
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
            Some(tk) => {
                let (token, kind) = tk;
                match kind {
                    TokenKind::Select => {
                        self.consume(TokenKind::Select);
                        let expr = self.parse_expression()?;
                        return Ok(ast::Expr::Statment(ast::Stmt::Select {
                            cols: Box::new(expr),
                        }));
                    }

                    TokenKind::Number(n) => {
                        self.consume(TokenKind::Number(n));
                        return Ok(ast::Expr::Number(n));
                    }

                    TokenKind::Init => return Ok(ast::Expr::Statment(ast::Stmt::Init)),

                    TokenKind::Create => {
                        self.consume(TokenKind::Create)?;
                        self.consume(TokenKind::Register)?;
                        let empty_string = String::new();
                        match self.peek_token() {
                            Some(token) => {
                                let (tk, kind) = token;
                                match kind {
                                    TokenKind::Identifier(name) => {
                                        return Ok(ast::Expr::Statment(ast::Stmt::Create {
                                            reg_name: name,
                                        }));
                                    }
                                    other => {
                                        return Err(ParserErr::ExpectedIdentifier {
                                            input: self.query.to_string(),
                                            givenkind: other,
                                            span: tk.span,
                                        });
                                    }
                                }
                            }
                            None => {
                                return Err(ParserErr::UnexpectedEndOfExpression {
                                    input: self.query.to_string(),
                                    tokind: TokenKind::Identifier(empty_string),
                                    span: Span {
                                        start: self.query.len(),
                                        end: self.query.len() + 1,
                                    },
                                });
                            }
                        }
                    }
                    _ => return Ok(ast::Expr::Empty),
                }
            }
            None => return Ok(ast::Expr::Empty),
        }
        Ok(ast::Expr::Empty)
    }
}
