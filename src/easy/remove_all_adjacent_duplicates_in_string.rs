#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn remove_duplicates(s: String) -> String {
        if s.len() <= 1 {
            return s;
        }
        let mut index: usize = 0;
        let mut s: Vec<char> = s.chars().collect();

        while s.len() > 1 && index < s.len() - 1 {
            if s[index] == s[index + 1] {
                s.remove(index);
                s.remove(index);
                if index > 0 {
                    index -= 1;
                }
                continue;
            }

            index += 1;
        }
        s.iter().collect()
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;
    #[test]
    fn solution_1_test_1() {
        let input = "abbaca".to_string();
        let expected = "ca".to_string();
        let res = Solution::remove_duplicates(input);
        assert_eq!(res, expected);
    }

    #[test]
    fn solution_1_test_2() {
        let input = "azxxzy".to_string();
        let expected = "ay".to_string();
        let res = Solution::remove_duplicates(input);
        assert_eq!(res, expected);
    }
}
