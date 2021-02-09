#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub content: String,
    pub position: TokenPosition,
}

#[derive(Debug)]
pub enum TokenType {
    Keyword,
    Number,
    String,
    Operator,
    Symbol,
    Separator,
}

#[derive(Debug)]
pub struct TokenPosition {
    pub start: usize,
    pub end: usize,
}