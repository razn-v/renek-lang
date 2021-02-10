#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub content: String,
    pub position: TokenPosition,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TokenType {
    Keyword,
    Number,
    String,
    Operator,
    Symbol,
    Separator,
}

#[derive(Debug, Clone)]
pub struct TokenPosition {
    pub start: usize,
    pub end: usize,
}