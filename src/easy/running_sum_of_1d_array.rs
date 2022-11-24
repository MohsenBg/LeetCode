pub struct Solution {}
impl Solution {
    // check this link for more information
    // https://www.youtube.com/watch?v=7pJo_rM0z_s
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        // make array to store sums
        let mut prefix_sum: Vec<i32> = Vec::new();
        // get last sum
        let mut last_sum = 0;
        for value in nums.iter() {
            last_sum += value;
            // add last_sum + new value to new sum
            prefix_sum.push(last_sum);
        }
        prefix_sum
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn solution_1_test_1() {
        let input = vec![1, 2, 3, 4];
        let output = vec![1, 3, 6, 10];
        assert_eq!(Solution::running_sum(input), output);
    }
    #[test]
    fn solution_1_test_2() {
        let input = vec![1, 1, 1, 1, 1];
        let output = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::running_sum(input), output);
    }

    #[test]
    fn solution_1_test_3() {
        let input = vec![3, 1, 2, 10, 1];
        let output = vec![3, 4, 6, 16, 17];
        assert_eq!(Solution::running_sum(input), output);
    }
}
