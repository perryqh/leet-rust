#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let node_rc = root.unwrap();
        let mut node_ref = node_rc.borrow_mut();
        node_ref.val.to_string()
            + &match (node_ref.left.take(), node_ref.right.take()) {
            (None, None) => String::new(),
            (left, None) => format!("({})", Self::tree2str(left)),
            (None, right) => format!("()({})", Self::tree2str(right)),
            (left, right) => format!("({})({})", Self::tree2str(left), Self::tree2str(right)),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut root = TreeNode::new(1);
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root.left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        assert_eq!(Solution::tree2str(Some(Rc::new(RefCell::new(root)))), "1(2(4))(3)".to_string());
    }

    #[test]
    fn test_2() {
        let mut root = TreeNode::new(1);
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root.left.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        assert_eq!(Solution::tree2str(Some(Rc::new(RefCell::new(root)))), "1(2()(4))(3)".to_string());
    }
}