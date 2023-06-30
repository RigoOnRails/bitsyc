use anyhow::{Result, bail};

use crate::lexer::{Lexer, Token};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    next_token: Option<Token>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Result<Self> {
        let Some(current_token) = lexer.next().transpose()? else {
            bail!("The program is empty.");
        };

        let next_token = lexer.next().transpose()?;

        Ok(Self {
            lexer,
            current_token,
            next_token,
        })
    }
}
