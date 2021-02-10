use std::fmt::Debug;

use crate::lexer::token::{Token, TokenType};
use crate::parser::parser::ParseError::PeekNone;
use crate::parser::tree::{BoolNode, NumberNode, ParseNode, ParseTree, StringNode, VariableNode};
use crate::parser::types::Type;

type Node = Option<Box<dyn ParseNode>>;

#[derive(Debug)]
pub enum ParseError {
    PeekNone,
}

pub struct Parser {
    tokens: Vec<Token>,
    tree: ParseTree,
    current_pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            tree: ParseTree::new(),
            current_pos: 0,
        }
    }

    pub fn parse(&mut self) {
        let node = self.parse_node();
        println!("{:?}", node);
    }

    fn step(&mut self, n: usize) -> bool {
        if self.peek(1).is_ok() {
            self.current_pos += n;
            return true;
        }
        false
    }

    fn peek(&self, steps: usize) -> Result<&Token, ParseError> {
        if self.current_pos + steps >= self.tokens.len() {
            return Err(PeekNone);
        }
        Ok(&self.tokens[self.current_pos + steps])
    }

    fn equals_type(&self, token_type: TokenType) -> bool {
        match self.peek(0) {
            Ok(token) => token.token_type == token_type,
            _ => false
        }
    }

    fn equals_content(&self, content: &str) -> bool {
        match self.peek(0) {
            Ok(token) => token.content == content,
            _ => false
        }
    }

    fn parse_node(&mut self) -> Node {
        // 6 possibilities :
        //  - variable declaration
        //  - function call
        //  - condition
        //  - loop
        //  - block 
        //  - statement (return, break...)
        let var_decl = self.parse_var_decl();
        if var_decl.is_some() {
            return var_decl;
        }
        None
    }

    fn parse_expr(&mut self) -> Node {
        // 4 possibilities :
        //  - operation
        //  - fixed value (boolean, integer...)
        //  - variable call
        //  - function call
        let value = self.parse_value();
        if value.is_some() {
            return value;
        }
        None
    }

    fn parse_value(&mut self) -> Node {
        if self.equals_type(TokenType::Keyword) {
            return self.parse_bool();
        } else if self.equals_type(TokenType::Number) {
            return self.parse_number();
        } else if self.equals_type(TokenType::String) {
            return self.parse_string();
        }
        None
    }

    fn parse_bool(&mut self) -> Node {
        if self.equals_content("True") {
            return Some(Box::new(BoolNode { value: true }));
        } else if self.equals_content("False") {
            return Some(Box::new(BoolNode { value: false }));
        }
        None
    }

    fn parse_number(&mut self) -> Node {
        Some(Box::new(NumberNode {
            int_value: self.peek(0).unwrap().content.parse::<usize>().unwrap()
        }))
    }

    fn parse_string(&mut self) -> Node {
        Some(Box::new(StringNode { value: self.peek(0).unwrap().content.clone() }))
    }

    fn parse_var_decl(&mut self) -> Node {
        if !self.equals_type(TokenType::Keyword) && !self.equals_content("var") {
            return None;
        }

        // Skip "var" keyword
        self.step(1);

        let var_name = self.peek(0).unwrap().content.clone();

        // Skip variable name
        self.step(1);

        if !self.equals_content("::") {
            return None;
        }

        // Skip "::" symbol
        self.step(1);

        if !self.equals_type(TokenType::Keyword) {
            println!("Invalid variable type");
            return None;
        }
 
        let var_type = match Type::from_token(self.peek(0).unwrap()) {
            Some(t) => t,
            None => {
                println!("Invalid variable type");
                return None;
            }
        };  
        
        // Skip variable type
        self.step(1);

        if !self.equals_content("=") {
            println!("Invalid operator");
            return None;
        }

        // Skip assign operator
        self.step(1);

        let var_value = self.parse_expr();
        return match var_value {
            None => {
                println!("Invalid variable value");
                None
            }
            Some(node) => Some(Box::new(VariableNode {
                var_type,
                name: var_name,
                value: node,
            }))
        };
    }
}
