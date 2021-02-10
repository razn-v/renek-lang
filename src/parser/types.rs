use crate::lexer::token::Token;

#[derive(Debug)]
pub enum Type {
    Int8,
    Int16,
    Int32,
    Int64,
    Float32,
    Float64,
    Bool,
    Char,
    String,
}

impl Type {
    pub fn from_token(token: &Token) -> Option<Self> {
        return match token.content.as_str() {
            "Int8" => Some(Self::Int8),
            "Int16" => Some(Self::Int16),
            "Int32" => Some(Self::Int32),
            "Int64" => Some(Self::Int64),
            "Float32" => Some(Self::Float32),
            "Float64" => Some(Self::Float64),
            "Bool" => Some(Self::Bool),
            "Char" => Some(Self::Char),
            "String" => Some(Self::String),
            _ => None
        }
    }
}
