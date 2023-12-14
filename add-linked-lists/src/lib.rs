// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
struct Solution;
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1_stack = Solution::build_stack(&l1);
        let mut l2_stack = Solution::build_stack(&l2);
        let mut head = None;
        let mut carry_over = 0;
        while carry_over != 0 || !l1_stack.is_empty() || !l2_stack.is_empty() {
            let total = carry_over + Solution::pop_or_zero(&mut l1_stack) + Solution::pop_or_zero(&mut l2_stack);
            carry_over = total / 10;
            let mut node = ListNode::new(total % 10);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }

    fn pop_or_zero(stack: &mut Vec<i32>) -> i32 {
        match stack.pop() {
            Some(val) => val,
            None => 0,
        }
    }

    fn build_stack(ll: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut head = ll;
        while let Some(node) = head {
            stack.push(node.val);
            head = &node.next;
        }
        stack
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn create_list_node_from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for i in v.iter().rev() {
            let mut node = ListNode::new(*i);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }

    fn create_vec_from_list_node(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut v = Vec::new();
        while let Some(node) = head {
            v.push(node.val);
            head = node.next;
        }
        v
    }

    #[test]
    fn three_digit_with_carry() {
        let l1 = create_list_node_from_vec(vec![2, 4, 3]);
        let l2 = create_list_node_from_vec(vec![5, 6, 4]);
        let expected = create_list_node_from_vec(vec![8,0,7]);
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }

    #[test]
    fn test_zeros() {
        let l1 = create_list_node_from_vec(vec![0]);
        let l2 = create_list_node_from_vec(vec![0]);
        let expected = create_list_node_from_vec(vec![0]);
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }

    #[test]
    fn test_large() {
        let l1 = create_list_node_from_vec(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = create_list_node_from_vec(vec![9, 9, 9, 9]);
        let expected = create_list_node_from_vec(vec![1,0,0,0,9,9,9,8]);
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }
}