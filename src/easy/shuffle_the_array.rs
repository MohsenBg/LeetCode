#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let (left_slice, right_slice) = nums.split_at(n as usize);

        let mut shuffle_arr = Vec::with_capacity(nums.len());
        for (&left, &right) in left_slice.iter().zip(right_slice.iter()) {
            shuffle_arr.push(left);
            shuffle_arr.push(right);
        }

        shuffle_arr
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    pub fn solution_1_test_1() {
        let input = vec![2, 5, 1, 3, 4, 7];
        let expected = vec![2, 3, 5, 4, 1, 7];
        let n = (input.len() / 2) as i32;
        let output = Solution::shuffle(input, n);
        assert_eq!(expected, output);
    }

    #[test]
    pub fn solution_1_test_2() {
        let input = vec![1, 2, 3, 4, 4, 3, 2, 1];
        let expected = vec![1, 4, 2, 3, 3, 2, 4, 1];
        let n = (input.len() / 2) as i32;
        let output = Solution::shuffle(input, n);
        assert_eq!(expected, output);
    }
}
