pub mod ast;
pub mod errs;
pub mod ops;
pub mod types;
pub mod vals;

use std::str::FromStr;

use crate::tree;

#[derive(PartialEq, Clone, Debug)]
pub struct RegExp {
    s: String,
    syntax_tree: ast::AbstractSyntaxTree,
    params_tree: tree::BinTree<ast::Params>,
}

impl RegExp {
    pub fn str(&self) -> &str {
        &self.s
    }

    pub fn syntax_tree(&self) -> &ast::AbstractSyntaxTree {
        &self.syntax_tree
    }

    pub fn params_tree(&self) -> &tree::BinTree<ast::Params> {
        &self.params_tree
    }
}

impl FromStr for RegExp {
    type Err = errs::ParseExpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let t = ast::AbstractSyntaxTree::from_str(s)?;
        Ok(RegExp {
            s: s.to_string(),
            syntax_tree: t,
            params_tree: t.params_tree(),
        })
    }
}
