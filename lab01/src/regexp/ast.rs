use super::{ops, vals};
use crate::tree::{BinTree, TreeNode};
use std::error::Error;
use std::{error, fmt};

#[derive(Debug)]
enum Symbol {
    Value(vals::Value),
    Operator(ops::Operator),
}

#[derive(PartialEq, Debug)]
pub struct AbstractSyntaxTree {
    root: BinTree<char>,
}

impl AbstractSyntaxTree {
    pub fn new() -> Self {
        AbstractSyntaxTree {
            root: BinTree::new(),
        }
    }
}
