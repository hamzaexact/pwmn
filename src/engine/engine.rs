use crate::error::LexerErr;
use crate::interpreter::Span;
use crate::interpreter::eval::eval;
use crate::interpreter::lexer;
use crate::interpreter::parser;
use crate::session::session_conn::SessionConn;
use crate::storage::init;

type DynamicError = Box<dyn std::error::Error>;

pub struct Executor {}

impl Executor {
    pub fn execute(input: &str, session: &mut SessionConn) -> Result<(), DynamicError> {
        let lexed_tokens = lexer::Lexer::tokenize(input)?;
        let parse_result = parser::Parser::parse(lexed_tokens)?;
        let eval = parse_result.eval(session)?;
        Ok(())
    }
}
