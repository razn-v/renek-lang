use crate::lexer::token::Token;

#[derive(Debug, Eq, PartialEq)]
pub enum Statement {
    Return,
    Break,
    Continue,
}

impl Statement {
    pub fn from_token(token: &Token) -> Option<Self> {
        return match token.content.as_str() {
            "return" => Some(Self::Return),
            "break" => Some(Self::Break),
            "continue" => Some(Self::Continue),
            _ => None,
        }
    }
}
