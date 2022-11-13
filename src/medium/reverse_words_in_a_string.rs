pub struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        // convert string to vector split with space
        let s: Vec<&str> = s.split(" ").collect();
        // make new string to store words
        let mut result = String::new();
        // loop reverse on the words
        for word in s.iter().rev() {
            //ignore empty word (spaces)
            if word.is_empty() {
                continue;
            }
            // add new  word
            result.push_str(word);
            // add space end it
            result.push(' ');
        }
        // remove last sapce
        result.pop();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn solution_1_test_1() {
        let input = "the sky is blue".to_string();
        let expcted = "blue is sky the".to_string();
        assert_eq!(Solution::reverse_words(input), expcted);
    }

    #[test]
    fn solution_1_test_2() {
        let input = "a good   example".to_string();
        let expcted = "example good a".to_string();
        assert_eq!(Solution::reverse_words(input), expcted);
    }

    #[test]
    fn solution_1_test_3() {
        let input = "  hello world  ".to_string();
        let expcted = "world hello".to_string();
        assert_eq!(Solution::reverse_words(input), expcted);
    }
}
