use std::collections::HashMap;
#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut num_counts = HashMap::new();

        for &num in nums.iter() {
            let entry = num_counts.entry(num).or_insert(0);
            *entry += 1;
        }

        for (_, appearing) in num_counts.iter() {
            // if we have [1,1,1,1] the answer is 6 right so we need a formal or pattern.
            // [1] , [1,1] , [1,1,1]   , [1,1,1,1]
            //  0  ,   1   ,    3      ,     6
            // n * (n - 1) / 2
            count += appearing * (appearing - 1) / 2;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    pub fn solution_1_test_1() {
        let input = vec![1, 2, 3, 1, 1, 3];
        let expected = 4;
        let output = Solution::num_identical_pairs(input);
        assert_eq!(expected, output);
    }

    #[test]
    pub fn solution_1_test_2() {
        let input = vec![1, 1, 1, 1];
        let expected = 6;
        let output = Solution::num_identical_pairs(input);
        assert_eq!(expected, output);
    }

    #[test]
    pub fn solution_1_test_3() {
        let input = vec![1, 2, 3];
        let expected = 0;
        let output = Solution::num_identical_pairs(input);
        assert_eq!(expected, output);
    }
}
