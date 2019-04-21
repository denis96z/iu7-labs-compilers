use std::str::FromStr;

use crate::{trees, types};

pub mod ast;
pub mod errs;
pub mod ops;
pub mod vals;

#[derive(PartialEq, Clone, Debug)]
pub struct RegExp {
    s: String,
    syntax_tree: ast::AbstractSyntaxTree,
    params_tree: trees::BinTree<ast::Params>,
}

impl RegExp {
    pub fn str(&self) -> &str {
        &self.s
    }

    pub fn syntax_tree(&self) -> &ast::AbstractSyntaxTree {
        &self.syntax_tree
    }

    pub fn params_tree(&self) -> &trees::BinTree<ast::Params> {
        &self.params_tree
    }

    pub fn extract_values(&self) -> Vec<(usize, &vals::Value, &types::Set<usize>)> {
        ast::extract_values(&self.syntax_tree.root(), &self.params_tree)
    }
}

impl FromStr for RegExp {
    type Err = errs::ParseExpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let augmented_regexp = augment_regexp(s);
        let t = ast::AbstractSyntaxTree::from_str(&augmented_regexp)?;
        Ok(RegExp {
            s: augmented_regexp,
            params_tree: t.make_params_tree(),
            syntax_tree: t,
        })
    }
}

fn augment_regexp(s: &str) -> String {
    format!("({})#", s)
}

mod tests {
    #[test]
    fn augment_regexp() {
        assert_eq!(super::augment_regexp("abc"), "(abc)#");
    }
}
