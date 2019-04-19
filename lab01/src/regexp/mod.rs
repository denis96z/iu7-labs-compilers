use std::str::FromStr;

pub mod ast;
pub mod error;
pub mod ops;
pub mod types;
pub mod vals;

#[derive(PartialEq, Clone, Debug)]
pub struct RegExp {
    s: String,
    tree: ast::AbstractSyntaxTree,
}

impl RegExp {
    pub fn str(&self) -> &str {
        &self.s
    }

    pub fn tree(&self) -> &ast::AbstractSyntaxTree {
        &self.tree
    }
}

impl FromStr for RegExp {
    type Err = error::ParseExpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RegExp {
            s: s.to_string(),
            tree: ast::AbstractSyntaxTree::from_str(s)?,
        })
    }
}
