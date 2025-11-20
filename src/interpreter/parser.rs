use crate::error::ParserErr;
use crate::interpreter::lexer::{LexResult, Span, Token, TokenKind};
use crate::interpreter::{ast, lexer};
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
        let k = parser.consume(TokenKind::Select);
        let f = parser.consume(TokenKind::As);
        match f {
            Ok(_) => return Ok(ast::Stmt::Empty),
            Err(e) => return Err(e),
        }
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
}
