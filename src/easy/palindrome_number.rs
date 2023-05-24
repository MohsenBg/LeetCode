#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn is_palindrome(x: i32) -> bool {
        //convert x to string
        let x = x.to_string();
        // reverce x
        let y = x.clone().chars().rev().collect::<String>();
        // check if reverce x same as self
        x == y
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn solution_1_test_1() {
        assert_eq!(Solution::is_palindrome(121), true)
    }

    #[test]
    fn solution_1_test_2() {
        assert_eq!(Solution::is_palindrome(-121), false)
    }

    #[test]
    fn solution_1_test_3() {
        assert_eq!(Solution::is_palindrome(10), false)
    }
}
