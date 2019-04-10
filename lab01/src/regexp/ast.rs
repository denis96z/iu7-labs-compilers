use crate::tree::{BinTree, TreeNode};

#[derive(PartialEq, Debug)]
pub struct AST {
    root: BinTree<char>,
}

impl AST {
    pub fn new() -> Self {
        AST {
            root: BinTree::new(),
        }
    }

    pub fn from_regexp() -> Self {
        unimplemented!()
    }
}

struct AstNode(TreeNode<char>);

impl AstNode {}
