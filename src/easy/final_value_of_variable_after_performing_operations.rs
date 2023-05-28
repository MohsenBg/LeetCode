#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut x = 0;
        for operation in operations {
            if operation.contains("++") {
                x += 1;
            } else if operation.contains("--") {
                x -= 1;
            }
        }
        x
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    pub fn solution_1_test_1() {
        let input: Vec<String> = vec!["--X", "X++", "X++"]
            .iter()
            .map(ToString::to_string)
            .collect();

        let expected = 1;
        let output = Solution::final_value_after_operations(input);
        assert_eq!(expected, output);
    }
    #[test]
    pub fn solution_1_test_2() {
        let input: Vec<String> = vec!["X++", "++X", "--X", "X--"]
            .iter()
            .map(ToString::to_string)
            .collect();

        let expected = 0;
        let output = Solution::final_value_after_operations(input);
        assert_eq!(expected, output);
    }
}
