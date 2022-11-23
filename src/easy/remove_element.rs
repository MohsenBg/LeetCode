pub struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        nums.sort();
        let mut index = Solution::binary_search(&nums, val);
        println!("{:?}", nums);
        println!("{}", index);
        match index {
            -1 => {}
            _ => {
                while index < nums.len() as i32 && nums[index as usize] == val {
                    nums.remove(index as usize);
                }
                index -= 1;
                while index >= 0 && nums[index as usize] == val {
                    nums.remove(index as usize);
                    index -= 1;
                }
            }
        }
        nums.len() as i32
    }

    pub fn binary_search(nums: &Vec<i32>, val: i32) -> i32 {
        let mut low: i32 = 0;
        let mut hight: i32 = (nums.len() as i32) - 1;
        while hight >= low {
            let mid = (low + hight) / 2;
            if val == nums[mid as usize] {
                return mid as i32;
            } else if val > nums[mid as usize] {
                low = mid + 1;
            } else if val < nums[mid as usize] {
                hight = mid - 1;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solution_1_test_1() {
        let mut input = vec![3, 2, 2, 3];
        let mut res = vec![2, 2];
        res.sort();
        let val = 3;
        Solution::remove_element(&mut input, val);
        assert_eq!(input, res);
    }

    #[test]
    fn solution_1_test_2() {
        let mut input = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let mut res = vec![0, 1, 4, 0, 3];
        res.sort();
        let val = 2;
        Solution::remove_element(&mut input, val);
        assert_eq!(input, res);
    }

    #[test]
    fn solution_1_test_3() {
        let mut input = vec![];
        let mut res = vec![];
        res.sort();
        let val = 0;
        Solution::remove_element(&mut input, val);
        assert_eq!(input, res);
    }

    #[test]
    fn solution_1_test_4() {
        let mut input = vec![2, 2, 2];
        let mut res = vec![2, 2, 2];
        res.sort();
        let val = 0;
        Solution::remove_element(&mut input, val);
        assert_eq!(input, res);
    }
}
