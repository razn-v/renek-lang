use std::fmt::Debug;

use crate::parser::types::Type;
use crate::parser::statements::Statement;

pub type Node = Box<dyn ParseNode>;

pub struct ParseTree {
    nodes: Vec<Box<dyn ParseNode>>,
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

// TODO: support floats and negative numbers
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

#[derive(Debug)]
pub struct FunctionCallNode {
    pub name: String,
    pub args: Vec<Node>,
}

impl ParseNode for FunctionCallNode {}

#[derive(Debug)]
pub struct VariableCallNode {
    pub name: String,
}

impl ParseNode for VariableCallNode {}

#[derive(Debug)]
pub struct BlockNode {
    pub nodes: Vec<Node>,
}

impl ParseNode for BlockNode {}

#[derive(Debug)]
pub struct FunctionNode {
    pub name: String,
    pub args: Vec<FunctionArgNode>,
    pub return_type: Type,
    pub block: Box<dyn ParseNode>,
}

impl ParseNode for FunctionNode {}

#[derive(Debug)]
pub struct FunctionArgNode {
    pub var_type: Type,
    pub name: String,
}

impl ParseNode for FunctionArgNode {}

#[derive(Debug)]
pub struct StatementNode {
    pub stat_type: Statement,
    // Only if the statement is 'return'
    pub value: Option<Box<dyn ParseNode>>,
}

impl ParseNode for StatementNode {}
