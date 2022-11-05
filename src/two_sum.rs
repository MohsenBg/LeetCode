#[derive(Debug)]
pub struct Solution {}
impl Solution {
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
