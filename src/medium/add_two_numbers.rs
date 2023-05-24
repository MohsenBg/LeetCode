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
#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let l1_str = into_string(l1).chars().rev().collect::<String>();
        let l2_str = into_string(l2).chars().rev().collect::<String>();

        let l1_vec = l1_str.chars().collect::<Vec<char>>();
        let l2_vec = l2_str.chars().collect::<Vec<char>>();
        println!("{:?}", &l1_vec);
        println!("{:?}", &l2_vec);

        let sum = sum_vec(l1_vec, l2_vec);

        println!("{:?}", &sum);

        return into_list(sum);
    }
}

// convert link list to string
pub fn into_string(head: Option<Box<ListNode>>) -> String {
    let mut str = String::new();
    let mut current_node = head;
    while current_node != None {
        let str_value = current_node.as_ref().unwrap().val.to_string();
        str.push_str(&str_value);
        current_node = current_node.unwrap().next;
    }
    str
}

//convert vec to link list
fn into_list(vector: Vec<i32>) -> Option<Box<ListNode>> {
    let mut cur = None;
    for &value in vector.iter().rev() {
        let mut new_node = ListNode::new(value);
        new_node.next = cur;
        cur = Some(Box::new(new_node));
    }

    return cur;
}

fn sum_vec(num1: Vec<char>, num2: Vec<char>) -> Vec<i32> {
    if num1.len() < num2.len() {
        return sum_vec(num2, num1);
    }

    let mut sum: Vec<i32> = Vec::new();
    let mut owerflow: u32 = 0;
    let mut c = num1.len() - num2.len();
    let mut num2 = num2;

    while c > 0 {
        num2.insert(0, '0');
        c -= 1;
    }

    println!("{:?}\n{:?}", num1, num2);
    let mut index = (num2.len() - 1) as i32;

    while index >= 0 {
        let v1 = num1[index as usize].to_digit(10).unwrap();
        let v2 = num2[index as usize].to_digit(10).unwrap();
        println!("{}+{}+{}={}", &v1, &v2, &owerflow, v1 + v2 + owerflow);
        let total = v1 + v2 + owerflow;
        owerflow = total / 10;
        sum.push((total % 10) as i32);
        index -= 1;
    }

    if owerflow > 0 {
        sum.push(owerflow as i32);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::into_list;
    use super::Solution;
    #[test]
    fn solution_1_test_1() {
        let l1 = into_list(vec![2, 4, 3]);
        let l2 = into_list(vec![5, 6, 4]);
        let expected_res = into_list(vec![7, 0, 8]);
        let res = Solution::add_two_numbers(l1, l2);
        assert_eq!(res, expected_res);
    }
    #[test]
    fn solution_1_test_2() {
        let l1 = into_list(vec![0]);
        let l2 = into_list(vec![0]);
        let expected_res = into_list(vec![0]);
        let res = Solution::add_two_numbers(l1, l2);
        assert_eq!(res, expected_res);
    }
    #[test]
    fn solution_1_test_3() {
        let l1 = into_list(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = into_list(vec![9, 9, 9, 9]);
        let expected_res = into_list(vec![8, 9, 9, 9, 0, 0, 0, 1]);
        let res = Solution::add_two_numbers(l1, l2);
        assert_eq!(res, expected_res);
    }
    #[test]
    fn solution_1_test_4() {
        let l1 = into_list(vec![2, 4, 9]);
        let l2 = into_list(vec![5, 6, 4, 9]);
        let expected_res = into_list(vec![7, 0, 4, 0, 1]);
        let res = Solution::add_two_numbers(l1, l2);
        println!("{:?}", vec![7, 0, 4, 0, 1]);
        assert_eq!(res, expected_res);
    }
}
