#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution {}
impl Solution {
    pub fn make_list(val: i32) -> Option<Box<ListNode>> {
        Some(Box::new(ListNode::new(val)))
    }

    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list1 = list1;
        let mut list2 = list2;

        if list1.is_none() {
            return list2;
        }

        if list2.is_none() {
            return list1;
        }

        let mut current_list1 = list1.as_mut();

        let mut current_list2 = list2.as_mut();

        let mut list: Option<Box<ListNode>> = None;
        {
            let mut value = 0;
            if current_list1.as_ref().unwrap().val < current_list2.as_ref().unwrap().val {
                value = current_list1.as_ref().unwrap().val;
                current_list1 = current_list1.unwrap().next.as_mut();
            } else {
                value = current_list2.as_ref().unwrap().val;
                current_list2 = current_list2.unwrap().next.as_mut();
            }
            list = Solution::make_list(value);
        }

        let mut current_list = list.as_mut();
        while current_list1.as_ref().is_some() && current_list2.as_ref().is_some() {
            let mut value = 0;
            if current_list1.as_ref().unwrap().val < current_list2.as_ref().unwrap().val {
                value = current_list1.as_ref().unwrap().val;
                current_list1 = current_list1.unwrap().next.as_mut();
            } else {
                value = current_list2.as_ref().unwrap().val;
                current_list2 = current_list2.unwrap().next.as_mut();
            }
            current_list.as_mut().unwrap().next = Solution::make_list(value);
            current_list = current_list.unwrap().next.as_mut();
        }

        while current_list1.as_ref().is_some() {
            let value = current_list1.as_ref().unwrap().val;
            current_list1 = current_list1.unwrap().next.as_mut();
            current_list.as_mut().unwrap().next = Solution::make_list(value);
            current_list = current_list.unwrap().next.as_mut();
        }

        while current_list2.as_ref().is_some() {
            let value = current_list2.as_ref().unwrap().val;
            current_list2 = current_list2.unwrap().next.as_mut();
            current_list.as_mut().unwrap().next = Solution::make_list(value);
            current_list = current_list.unwrap().next.as_mut();
        }
        list
    }
}

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

pub fn list_to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vector: Vec<i32> = Vec::new();
    let mut list = list;
    while list != None {
        vector.push(list.as_ref().unwrap().val);
        list = list.unwrap().next;
    }
    vector
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn solution_1_test_1() {
        let list1 = vec![1, 2, 4];
        let list2 = vec![1, 3, 4];
        let l1 = vec_to_list(list1);
        let l2 = vec_to_list(list2);
        let res = Solution::merge_two_lists(l1, l2);
        assert_eq!(list_to_vec(res), vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    pub fn solution_1_test_2() {
        let list1 = vec![];
        let list2 = vec![];
        let l1 = vec_to_list(list1);
        let l2 = vec_to_list(list2);
        let res = Solution::merge_two_lists(l1, l2);
        assert_eq!(list_to_vec(res), vec![]);
    }

    #[test]
    pub fn solution_1_test_3() {
        let list1 = vec![];
        let list2 = vec![0];
        let l1 = vec_to_list(list1);
        let l2 = vec_to_list(list2);
        let res = Solution::merge_two_lists(l1, l2);
        assert_eq!(list_to_vec(res), vec![0]);
    }
}
