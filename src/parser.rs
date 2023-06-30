use crate::lexer::{Lexer, Token};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    next_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next().unwrap().unwrap();
        let next_token = lexer.next().unwrap().unwrap();

        Self {
            lexer,
            current_token,
            next_token,
        }
    }
}
