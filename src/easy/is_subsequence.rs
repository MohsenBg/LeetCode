#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn is_subsequence(s: String, t: String) -> bool {
        // if len <= 0 than no need to check for subsequences
        // every string subsequence of zero
        if s.len() <= 0 {
            return true;
        }
        // convert string subsequence to vector of char
        let sub = s.chars().collect::<Vec<char>>();
        // index of subsequences
        let mut index = 0;
        // loop char from t string (main string)
        for ch in t.chars() {
            // check is char t equal to current index of char subsequence
            // if this true it's means we found one our char in t string
            if ch == sub[index] {
                // increase index of subsequence
                index += 1;
                // check if we our index is end of the subsequence
                // it's means all char in our subsequence founded
                if index == s.len() {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn solution_1_test_1() {
        let s = "abc".to_string();
        let t = "ahbgdc".to_string();
        assert_eq!(Solution::is_subsequence(s, t), true)
    }
    #[test]
    fn solution_1_test_2() {
        let s = "axc".to_string();
        let t = "ahbgdc".to_string();
        assert_eq!(Solution::is_subsequence(s, t), false)
    }
}
