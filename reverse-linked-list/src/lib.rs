struct Solution;

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
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut prev = None;

        while let Some(ref mut node) = head {
            let next = node.next.take();
            node.next = prev;
            prev = head;
            head = next;
        }
        prev
    }
}

// 1, prev None
// node.next = prev;
// prev = Some(node);
// head = head.unwrap().next;

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
    fn test_1() {
        let sol = Solution::reverse_list(create_list_node_from_vec(vec![1, 2, 3]));
        assert_eq!(create_vec_from_list_node(sol), vec![3, 2, 1]);
    }

    #[test]
    fn test_2() {
        let sol = Solution::reverse_list(create_list_node_from_vec(vec![0]));
        assert_eq!(create_vec_from_list_node(sol), vec![0]);
    }

    #[test]
    fn test_3() {
        let sol = Solution::reverse_list(create_list_node_from_vec(vec![1, 2]));
        assert_eq!(create_vec_from_list_node(sol), vec![2, 1]);
    }
}