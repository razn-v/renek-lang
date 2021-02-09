use std::fmt;

use crate::lexer::lexer::LexerError::{InvalidString, PeekNone};
use crate::lexer::token::{Token, TokenPosition, TokenType};

#[derive(Debug, Eq, PartialEq)]
pub enum LexerError {
    PeekNone,
    InvalidString,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexerError::PeekNone => write!(f, "Peek out of range!"),
            LexerError::InvalidString => write!(f, "Invalid string! Did you forget a quote?")
        }
    }
}

pub struct Lexer {
    tokens: Vec<Token>,
    input: String,
    current_token_start: usize,
    current_pos: usize,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            input: String::new(),
            current_token_start: 0,
            current_pos: 0,
        }
    }

    pub fn lex(&mut self, input: String) -> Result<&Vec<Token>, LexerError> {
        self.input = input;

        loop {
            self.current_token_start = self.current_pos;

            if self.is_letter(self.peek(0)) {
                self.get_keyword_token();
            } else if self.is_digit(self.peek(0)) {
                self.get_number_token();
            } else if self.is_string(self.peek(0)) {
                match self.get_string_token() {
                    Err(err) => return Err(err),
                    _ => {}
                }
            } else if self.is_operator() {
                self.get_operator_token();
            } else if self.is_symbol() {
                self.get_symbol_token();
            } else if self.is_separator() {
                self.get_separator_token();
            }

            if !self.step() {
                break;
            }
        }

        Ok(&self.tokens)
    }

    fn step(&mut self) -> bool {
        if self.peek(1).is_ok() {
            self.current_pos += 1;
            return true;
        }
        false
    }

    fn peek(&self, steps: usize) -> Result<char, LexerError> {
        if self.current_pos + steps >= self.input.len() {
            return Err(PeekNone);
        }
        return Ok(self.input.as_bytes()[self.current_pos + steps] as char);
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token {
            token_type,
            content: self.input[self.current_token_start..self.current_pos + 1].to_string(),
            position: TokenPosition { start: self.current_token_start, end: self.current_pos },
        })
    }

    fn is_letter(&self, peek: Result<char, LexerError>) -> bool {
        peek.unwrap_or_default().is_ascii_alphabetic()
    }

    fn is_digit(&self, peek: Result<char, LexerError>) -> bool {
        peek.unwrap_or_default().is_ascii_digit()
    }

    fn is_string(&self, peek: Result<char, LexerError>) -> bool {
        peek == Ok('"')
    }

    fn is_operator(&self) -> bool {
        match self.peek(0) {
            Ok('=') | Ok('+') | Ok('-') |  Ok('*') | Ok('/') | Ok('%') | Ok('>') | Ok('<') => true,
            _ => false,
        }
    }

    fn is_symbol(&self) -> bool {
        return (self.peek(0) == Ok(':') && self.peek(1) == Ok(':'))     // ::
            || (self.peek(0) == Ok('-') && self.peek(1) == Ok('>'));    // ->
    }

    fn is_separator(&self) -> bool {
        match self.peek(0) {
            Ok('\n') | Ok(',') | Ok('(') | Ok(')') |  Ok('{') | Ok('}') => true,
            _ => false
        }
    }

    fn get_keyword_token(&mut self) {
        while self.is_letter(self.peek(1)) {
            self.step();
        }
        self.add_token(TokenType::Keyword);
    }

    fn get_number_token(&mut self) {
        while self.is_digit(self.peek(1)) {
            self.step();
        }
        self.add_token(TokenType::Number);
    }

    fn get_string_token(&mut self) -> Result<(), LexerError> {
        self.step();

        while self.peek(0) != Ok('"') {
            if !self.step() {
                return Err(InvalidString);
            }
        }

        self.add_token(TokenType::String);
        Ok(())
    }

    fn get_operator_token(&mut self) {
        if self.peek(1) == Ok('=') {
            self.step();
        }
        self.add_token(TokenType::Operator)
    }

    fn get_symbol_token(&mut self) {
        self.step();
        self.add_token(TokenType::Symbol);
    }

    fn get_separator_token(&mut self) {
        self.add_token(TokenType::Separator);
    }
}