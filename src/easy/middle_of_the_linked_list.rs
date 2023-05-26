#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut middle = &head;
        let mut traveler = &head;

        while traveler.as_ref().is_some() && traveler.as_ref().unwrap().next.as_ref().is_some() {
            middle = &middle.as_ref().unwrap().as_ref().next;
            traveler = &traveler.as_ref().unwrap().next.as_ref().unwrap().next;
        }

        middle.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::ListNode;
    #[allow(unused_imports)]
    use super::Solution;
    #[test]
    pub fn solution_1_test_1() {
        let input = vec_to_list(vec![1, 2, 3, 4, 5]);
        let expected = vec_to_list(vec![3, 4, 5]);
        let output = Solution::middle_node(input);
        assert_eq!(expected, output);
    }

    #[test]
    pub fn solution_1_test_2() {
        let input = vec_to_list(vec![1, 2, 3, 4, 5, 6]);
        let expected = vec_to_list(vec![4, 5, 6]);
        let output = Solution::middle_node(input);
        assert_eq!(expected, output);
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
}
