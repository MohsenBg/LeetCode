#[derive(Debug)]
pub struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn solution_1(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0;
        while i < nums.len() {
            let mut j = i + 1;
            while j < nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
                j += 1;
            }
            i += 1;
        }
        return vec![];
    }
    #[allow(dead_code)]
    pub fn solution_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut n: i32 = nums.len() as i32 - 1;
        let mut nums = nums;
        while n >= 0 {
            let t: i32 = target - nums.pop().unwrap();
            for (index, value) in nums.iter().enumerate() {
                if *value == t {
                    return vec![index as i32, n];
                }
            }
            n -= 1;
        }
        vec![]
    }
}

#[cfg(test)]
mod two_sum {
    use super::Solution;
    #[test]
    fn solution_1_test_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let res = Solution::solution_1(nums, target);
        assert_eq!(res, vec![0, 1]);
    }
    #[test]
    fn solution_1_test_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let res = Solution::solution_1(nums, target);
        assert_eq!(res, vec![1, 2]);
    }
    #[test]
    fn solution_1_test_3() {
        let nums = vec![3, 3];
        let target = 6;
        let res = Solution::solution_1(nums, target);
        assert_eq!(res, vec![0, 1]);
    }
}
