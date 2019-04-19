#[derive(PartialEq, Clone, Debug)]
pub enum BinTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

impl<T> BinTree<T> {
    pub fn new() -> Self {
        BinTree::Empty
    }

    pub fn from(element: T, left: BinTree<T>, right: BinTree<T>) -> Self {
        BinTree::NonEmpty(Box::new(TreeNode::from(element, left, right)))
    }

    pub fn from_element(element: T) -> Self {
        BinTree::NonEmpty(Box::new(TreeNode::new(element)))
    }

    pub fn from_elements(element: T, left: T, right: T) -> Self {
        BinTree::from(
            element,
            BinTree::from_element(left),
            BinTree::from_element(right),
        )
    }

    pub fn from_element_with_left(element: T, left: T) -> Self {
        BinTree::from(element, BinTree::from_element(left), BinTree::Empty)
    }

    pub fn from_element_with_right(element: T, right: T) -> Self {
        BinTree::from(element, BinTree::Empty, BinTree::from_element(right))
    }

    pub fn is_empty(&self) -> bool {
        match *self {
            BinTree::Empty => true,
            _ => false,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct TreeNode<T> {
    element: T,
    left: BinTree<T>,
    right: BinTree<T>,
}

impl<T> TreeNode<T> {
    pub fn new(element: T) -> Self {
        TreeNode {
            element,
            left: BinTree::Empty,
            right: BinTree::Empty,
        }
    }

    pub fn from(element: T, left: BinTree<T>, right: BinTree<T>) -> Self {
        TreeNode {
            element,
            left,
            right,
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn tree_node_new() {
        let c = 'a';

        assert_eq!(
            TreeNode::new(c),
            TreeNode {
                element: c,
                left: BinTree::Empty,
                right: BinTree::Empty
            }
        )
    }

    #[test]
    fn tree_node_from() {
        let c = 'a';
        let left = BinTree::Empty;
        let right = BinTree::Empty;

        assert_eq!(
            TreeNode::from(c, left.clone(), right.clone()),
            TreeNode {
                element: c,
                left,
                right
            }
        )
    }

    #[test]
    fn bin_tree_new() {
        assert_eq!(BinTree::<char>::new(), BinTree::Empty);
    }

    #[test]
    fn bin_tree_from_element() {
        let element = 'a';

        match BinTree::from_element(element) {
            BinTree::Empty => panic!(),
            BinTree::NonEmpty(v) => {
                assert_eq!(v.element, element);
                assert_eq!(v.left, BinTree::Empty);
                assert_eq!(v.right, BinTree::Empty);
            }
        }
    }

    #[test]
    fn bin_tree_from_element_with_left() {
        let (element, left) = ('a', 'b');

        match BinTree::from_element_with_left(element, left) {
            BinTree::Empty => panic!(),
            BinTree::NonEmpty(v) => {
                assert_eq!(v.element, element);
                assert_eq!(v.left, BinTree::from_element(left));
                assert_eq!(v.right, BinTree::Empty);
            }
        }
    }

    #[test]
    fn bin_tree_from_element_with_right() {
        let (element, right) = ('a', 'b');

        match BinTree::from_element_with_right(element, right) {
            BinTree::Empty => panic!(),
            BinTree::NonEmpty(v) => {
                assert_eq!(v.element, element);
                assert_eq!(v.left, BinTree::Empty);
                assert_eq!(v.right, BinTree::from_element(right));
            }
        }
    }

    #[test]
    fn bin_tree_from_elements() {
        let (element, left, right) = ('a', 'b', 'c');

        match BinTree::from_elements(element, left, right) {
            BinTree::Empty => panic!(),
            BinTree::NonEmpty(v) => {
                assert_eq!(v.element, element);
                assert_eq!(v.left, BinTree::from_element(left));
                assert_eq!(v.right, BinTree::from_element(right));
            }
        }
    }

    #[test]
    fn bin_tree_is_empty() {
        assert_eq!(BinTree::<char>::Empty.is_empty(), true);
        assert_eq!(BinTree::from_element('a').is_empty(), false);
    }
}
