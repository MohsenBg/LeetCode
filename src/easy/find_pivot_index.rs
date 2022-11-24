pub struct Solution {}
impl Solution {
    // for more information check link below
    // https://www.youtube.com/watch?v=0PiH6Beqif8
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut left_sum = 0;
        let mut total_sum: i32 = 0;
        for value in nums.iter() {
            total_sum += value;
        }

        for (index, value) in nums.iter().enumerate() {
            if left_sum == total_sum - left_sum - value {
                return index as i32;
            }
            left_sum += value;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn solution_1_test_1() {
        let input = vec![1, 7, 3, 6, 5, 6];
        assert_eq!(Solution::pivot_index(input), 3);
    }
    #[test]
    fn solution_1_test_2() {
        let input = vec![1, 2, 3];
        assert_eq!(Solution::pivot_index(input), -1);
    }

    #[test]
    fn solution_1_test_3() {
        let input = vec![2, 1, -1];
        assert_eq!(Solution::pivot_index(input), 0);
    }
}
