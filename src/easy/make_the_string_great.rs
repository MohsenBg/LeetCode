#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn make_good(s: String) -> String {
        let mut s: Vec<char> = s.chars().collect();
        loop {
            // we use this varable to track if our string change or not
            // why: beacuse if it's not change and we not remove any char it's a good string
            let mut did_remove = false;
            let mut index: usize = 0;

            while s.len() != 0 && index < s.len() - 1 {
                // diffrent between uppercase and lowercase of the same char is 32
                // check two char is uppercase and lowercase of each other
                // use i8 beacuse biggest asscii code is 255
                if i8::abs(s[index] as i8 - s[index + 1] as i8) == 32 {
                    // beacuse we remove to char so we didn't need to move to increase index
                    // s.len = 10 => remove two element => s.len = 8 => we are already next index
                    s.remove(index);
                    s.remove(index);
                    // set did_remove = true beacuse we remove char from main string
                    did_remove = true;
                    continue;
                }
                // if move next index if not remove any char
                index += 1;
            }

            // we didn't remove any char it's means our string is good
            if !did_remove {
                break;
            }
        }

        s.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn solution_1_test_1() {
        let input = "leEeetcode".to_string();
        let expected = "leetcode".to_string();
        assert_eq!(Solution::make_good(input), expected);
    }

    #[test]
    fn solution_1_test_2() {
        let input = "abBAcC".to_string();
        let expected = "".to_string();
        assert_eq!(Solution::make_good(input), expected);
    }

    #[test]
    fn solution_1_test_3() {
        let input = "s".to_string();
        let expected = "s".to_string();
        assert_eq!(Solution::make_good(input), expected);
    }

    #[test]
    fn solution_1_test_4() {
        let input = "Pp".to_string();
        let expected = "".to_string();
        assert_eq!(Solution::make_good(input), expected);
    }
}
