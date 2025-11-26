use crate::error::LexerErr;
use crate::interpreter::Span;
use crate::interpreter::eval::eval;
use crate::interpreter::lexer;
use crate::interpreter::parser;
use crate::session::offSessionConn::OffSessionConn;
use crate::storage::init;
use std::error::Error;
type DynamicError = Box<dyn std::error::Error>;

pub struct Executor {}

impl Executor {
    pub fn execute(
        input: &str,
        session: &mut OffSessionConn,
    ) -> Result<(), DynamicError> {
        let lexed_tokens = lexer::Lexer::tokenize(input)?;
        let parse_result = parser::Parser::parse(lexed_tokens)?;
        let eval = parse_result.eval()?;
        Ok(())
    }
}
