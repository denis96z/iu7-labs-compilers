use std::error::Error;
use std::str::FromStr;
use std::{error, fmt};

use crate::tree::{BinTree, TreeNode};

use super::error::ParseExpError;
use super::{ops, vals};

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

impl FromStr for AbstractSyntaxTree {
    type Err = ParseExpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}
