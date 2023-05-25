#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn number_of_steps(num: i32) -> i32 {
        let mut i: i32 = 0;
        let mut num = num;
        loop {
            if num == 0 {
                break;
            }
            if 1 == num & 1 {
                num -= 1;
            } else {
                num /= 2;
            }
            i += 1;
        }
        i
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    pub fn solution_1_test_1() {
        let input = 14;
        let expected = 6;
        let output = Solution::number_of_steps(input);
        assert_eq!(expected, output);
    }

    #[test]
    pub fn solution_1_test_2() {
        let input = 8;
        let expected = 4;
        let output = Solution::number_of_steps(input);
        assert_eq!(expected, output);
    }

    #[test]
    pub fn solution_1_test_3() {
        let input = 123;
        let expected = 12;
        let output = Solution::number_of_steps(input);
        assert_eq!(expected, output);
    }
}
