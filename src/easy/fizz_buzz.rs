#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut result = Vec::with_capacity(n as usize);

        for i in 1..=n {
            let mut text = String::new();

            if i % 3 == 0 {
                text.push_str("Fizz");
            }

            if i % 5 == 0 {
                text.push_str("Buzz");
            }

            if text.is_empty() {
                text = i.to_string();
            }

            result.push(text);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::Solution;
    #[test]
    pub fn solution_1_test_1() {
        let input = 3;
        let expected: Vec<String> = vec!["1", "2", "Fizz"]
            .iter()
            .map(ToString::to_string)
            .collect();
        let output = Solution::fizz_buzz(input);

        assert_eq!(expected, output);
    }
}
