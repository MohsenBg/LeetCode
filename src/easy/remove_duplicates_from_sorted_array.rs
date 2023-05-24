use std::collections::HashSet;
#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut used_nums: HashSet<i32> = HashSet::new();
        let mut index = 0;
        while index < nums.len() {
            match used_nums.get(&nums[index]) {
                None => {
                    used_nums.insert(nums[index]);
                }
                Some(_) => {
                    nums.remove(index);
                    continue;
                }
            }
            index += 1;
        }
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn solution_1_test_1() {
        let mut input: Vec<i32> = vec![1, 1, 2];
        let expected: i32 = 2;
        let res = Solution::remove_duplicates(&mut input);
        assert_eq!(res, expected);
        assert_eq!(vec![1, 2], input);
    }

    #[test]
    fn solution_1_test_2() {
        let mut input: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let expected: i32 = 5;
        let res = Solution::remove_duplicates(&mut input);
        assert_eq!(res, expected);
        assert_eq!(vec![0, 1, 2, 3, 4], input);
    }
}
