#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut new_arr = Vec::with_capacity(nums.len());

        for &num in nums.iter() {
            new_arr.push(nums[num as usize])
        }

        new_arr
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::Solution;
    #[test]
    pub fn solution_1_test_1() {
        let input = vec![0, 2, 1, 5, 3, 4];
        let expected = vec![0, 1, 2, 4, 5, 3];
        let output = Solution::build_array(input);
        assert_eq!(expected, output);
    }

    #[test]
    pub fn solution_1_test_2() {
        let input = vec![5, 0, 1, 2, 3, 4];
        let expected = vec![4, 5, 0, 1, 2, 3];
        let output = Solution::build_array(input);
        assert_eq!(expected, output);
    }
}
