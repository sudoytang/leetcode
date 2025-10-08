// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
  pub fn new_with(val: i32, next: ListNode) -> Self {
    Self {
      val,
      next: Some(Box::new(next)),
    }
  }
}