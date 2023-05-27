#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut ans: Vec<i32> = Vec::with_capacity(len * 2);

        for _ in 0..2 {
            ans.extend(nums.iter().cloned());
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    pub fn solution_1_test_1() {
        let input = vec![1, 2, 1];
        let expected = vec![1, 2, 1, 1, 2, 1];
        let output = Solution::get_concatenation(input);
        assert_eq!(expected, output);
    }

    #[test]
    pub fn solution_1_test_2() {
        let input = vec![1, 3, 2, 1];
        let expected = vec![1, 3, 2, 1, 1, 3, 2, 1];
        let output = Solution::get_concatenation(input);
        assert_eq!(expected, output);
    }
}
