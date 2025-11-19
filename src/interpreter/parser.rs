// use crate::interpreter::ast;
//
// use crate::interpreter::lexer::Tokens;
//
// pub enum ParseErr {}
//
// pub struct Parser<'t> {
//     tokens: Vec<Tokens>,
//     tTokens: std::iter::Peekable<std::slice::Iter<'t, Tokens>>,
//     idx: usize,
// }
// impl<'t> Parser<'t> {
//     pub fn parse(tokens: Vec<Tokens>) -> Result<ast::Stmt, ParseErr> {
//         let mut pP = Parser {
//             tokens: tokens.clone(),
//             tTokens: tokens.iter().peekable(),
//             idx: 0,
//         };
//         match pP.parse_stmt() {
//             Ok(stmt) => Ok(stmt),
//             Err(e) => return Err(e),
//         }
//     }
//
//     fn parse_stmt(&mut self) -> Result<ast::Stmt, ParseErr> {
//         let mut tokens = self.tokens.iter().peekable();
//         let expr = self.depth1();
//         Ok(ast::Stmt::Empty)
//     }
//
//     fn depth1(&mut self) -> Result<ast::Stmt, ParseErr> {
//         let expr = self.depth1()?;
//         Ok(ast::Stmt::Empty)
//     }
//
//     fn root(&mut self) -> Result<ast::Stmt, ParseErr> {
//         Ok(ast::Stmt::Empty)
//     }
// }
