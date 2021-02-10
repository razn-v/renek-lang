use std::fmt::Debug;
use crate::parser::types::Type;

pub struct ParseTree {
    nodes: Vec<Box<dyn ParseNode>>
}

impl ParseTree {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new()
        }
    }
}

pub trait ParseNode: Debug {}

#[derive(Debug)]
pub struct VariableNode {
    pub var_type: Type,
    pub name: String,
    pub value: Box<dyn ParseNode>,
}

impl ParseNode for VariableNode {}

#[derive(Debug)]
pub struct BoolNode {
    pub value: bool,
}

impl ParseNode for BoolNode {}

// TODO: support floats
#[derive(Debug)]
pub struct NumberNode {
    pub int_value: usize,
}

impl ParseNode for NumberNode {}

#[derive(Debug)]
pub struct StringNode {
    pub value: String,
}

impl ParseNode for StringNode {}