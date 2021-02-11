use std::fmt::Debug;

use crate::lexer::token::{Token, TokenType};
use crate::parser::{
    tree::{
        BoolNode, FunctionCall, Node, NumberNode, ParseNode, ParseTree, StringNode, VariableCall,
        VariableNode,
    },
    types::Type,
};

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
        if self.peek(1).is_some() {
            self.current_pos += n;
            return true;
        }
        false
    }

    fn peek(&self, steps: usize) -> Option<&Token> {
        if self.current_pos + steps >= self.tokens.len() {
            return None;
        }
        Some(&self.tokens[self.current_pos + steps])
    }

    fn equals_type(&self, token_type: TokenType) -> bool {
        match self.peek(0) {
            Some(token) => token.token_type == token_type,
            _ => false
        }
    }

    fn equals_content(&self, content: &str) -> bool {
        match self.peek(0) {
            Some(token) => token.content == content,
            _ => false
        }
    }

    fn is_forbidden_keyword(&self) -> bool {
        match self.peek(0) {
            Some(token) => {
                // If the keyword can be parsed as a type,
                // it's a forbidden keyword
                if Type::from_token(token).is_some() {
                    return true;
                }

                return match token.content.as_str() {
                    "var" | "if" | "else" | "return" => true,
                    _ => false,
                };
            }
            _ => false
        }
    }

    fn expected(&self, what: &str) {
        println!("Expected {}, got {}", what, self.peek(0).unwrap().content);
    }

    fn parse_node(&mut self) -> Option<Node> {
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

        let func_call = self.parse_func_call();
        if func_call.is_some() {
            return func_call;
        }

        None
    }

    fn parse_expr(&mut self) -> Option<Node> {
        // 4 possibilities :
        //  - operation
        //  - fixed value (boolean, integer...)
        //  - variable call
        //  - function call
        let value = self.parse_value();
        if value.is_some() {
            return value;
        }

        let func_call = self.parse_func_call();
        if func_call.is_some() {
            return func_call;
        }

        let var_call = self.parse_var_call();
        if var_call.is_some() {
            return var_call;
        }

        None
    }

    fn parse_value(&mut self) -> Option<Node> {
        if self.equals_type(TokenType::Keyword) {
            return self.parse_bool();
        } else if self.equals_type(TokenType::Number) {
            return self.parse_number();
        } else if self.equals_type(TokenType::String) {
            return self.parse_string();
        }
        None
    }

    fn parse_bool(&mut self) -> Option<Node> {
        if self.equals_content("True") {
            return Some(Box::new(BoolNode { value: true }));
        } else if self.equals_content("False") {
            return Some(Box::new(BoolNode { value: false }));
        }
        None
    }

    fn parse_number(&mut self) -> Option<Node> {
        Some(Box::new(NumberNode {
            int_value: self.peek(0).unwrap().content.parse::<usize>().unwrap()
        }))
    }

    fn parse_string(&mut self) -> Option<Node> {
        Some(Box::new(StringNode { value: self.peek(0).unwrap().content.clone() }))
    }

    fn parse_var_decl(&mut self) -> Option<Node> {
        if !self.equals_content("var") {
            return None;
        }

        let init_pos = self.current_pos;

        // Skip "var" keyword
        self.step(1);

        if !self.equals_type(TokenType::Keyword) || self.is_forbidden_keyword() {
            println!("Invalid variable name");
            self.current_pos = init_pos;

            return None;
        }

        let var_name = self.peek(0).unwrap().content.clone();

        // Skip variable name
        self.step(1);

        if !self.equals_content("::") {
            self.expected("::");
            self.current_pos = init_pos;

            return None;
        }

        // Skip "::" symbol
        self.step(1);

        if !self.equals_type(TokenType::Keyword) {
            println!("Invalid variable type");
            self.current_pos = init_pos;

            return None;
        }

        let var_type = match Type::from_token(self.peek(0).unwrap()) {
            Some(t) => t,
            None => {
                println!("Invalid variable type");
                self.current_pos = init_pos;

                return None;
            }
        };

        // Skip variable type
        self.step(1);

        if !self.equals_content("=") {
            self.expected("=");
            self.current_pos = init_pos;

            return None;
        }

        // Skip assign operator
        self.step(1);

        let var_value = self.parse_expr();
        return match var_value {
            None => {
                println!("Invalid variable value");
                self.current_pos = init_pos;

                None
            }
            Some(node) => Some(Box::new(VariableNode {
                var_type,
                name: var_name,
                value: node,
            }))
        };
    }

    fn parse_var_call(&mut self) -> Option<Node> {
        if !self.equals_type(TokenType::Keyword) || self.is_forbidden_keyword() {
            return None;
        }

        Some(Box::new(VariableCall {
            name: self.peek(0).unwrap().content.clone()
        }))
    }

    fn parse_func_call(&mut self) -> Option<Node> {
        if !self.equals_type(TokenType::Keyword) || self.is_forbidden_keyword() {
            return None;
        }

        let init_pos = self.current_pos;
        let func_name = self.peek(0).unwrap().content.clone();

        // Skip function name
        self.step(1);

        if !self.equals_content("(") {
            self.current_pos = init_pos;
            return None;
        }

        // Skip opened parenthesis
        self.step(1);

        // Parse function arguments
        let mut func_args = Vec::<Node>::new();

        while self.peek(0).is_some() && !self.equals_content(")") {
            match self.parse_expr() {
                None => println!("Invalid function parameter '{}'",
                                 self.peek(0).unwrap().content),
                Some(arg) => func_args.push(arg)
            }

            self.step(1);
            if self.equals_content(",") {
                self.step(1);
            }
        }

        Some(Box::new(FunctionCall {
            name: func_name,
            args: func_args,
        }))
    }
}