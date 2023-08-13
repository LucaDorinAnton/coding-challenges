//Definition for singly-linked list.

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

pub enum Solution {}

impl Solution {

    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut nums: Vec<i32> = vec![];
        loop {
            match head {
                None => break,
                Some(b) => {
                    if (*b).val != val {
                        nums.push((*b).val);
                    }
                    head = (*b).next;
                }
            }
        }
        let mut cur: Option<Box<ListNode>> = None;
        for i in nums.iter().rev() {
            let mut new = ListNode::new(*i);
            new.next = cur;
            cur = Some(Box::new(new));
        }
        cur
    }

}
