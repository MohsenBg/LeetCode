// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // more information
        // https://www.youtube.com/watch?v=4IJKoqUVgFI

        let mut list = head;
        let mut prev: Option<Box<ListNode>> = None;

        while let Some(mut node) = list {
            list = node.next;
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}
#[allow(dead_code)]
pub fn vec_to_list(vector: Vec<i32>) -> Option<Box<ListNode>> {
    if vector.len() == 0 {
        return None;
    }
    let mut index: usize = 1;
    let mut list = Some(Box::new(ListNode::new(vector[0])));
    let mut current = list.as_mut();
    while index < vector.len() {
        let new_list = Some(Box::new(ListNode::new(vector[index])));
        current.as_mut().unwrap().next = new_list;
        current = current.unwrap().next.as_mut();
        index += 1;
    }
    list
}
#[allow(dead_code)]
pub fn list_to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vector: Vec<i32> = Vec::new();
    let mut list = list;
    while let Some(node) = list {
        vector.push(node.val);
        list = node.next;
    }
    vector
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn solution_1_test_1() {
        let list = vec![1, 2, 3, 4, 5];
        let l1 = vec_to_list(list);
        let res = Solution::reverse_list(l1);
        assert_eq!(list_to_vec(res), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    pub fn solution_1_test_2() {
        let list = vec![1, 2];
        let l1 = vec_to_list(list);
        let res = Solution::reverse_list(l1);
        assert_eq!(list_to_vec(res), vec![2, 1]);
    }

    #[test]
    pub fn solution_1_test_3() {
        let list = vec![];
        let l1 = vec_to_list(list);
        let res = Solution::reverse_list(l1);
        assert_eq!(list_to_vec(res), vec![]);
    }
}
