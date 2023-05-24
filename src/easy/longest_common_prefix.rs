#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result = "".to_string();

        // result empty string if have empty word or empty vec
        if strs.len() == 0 || strs[0].len() == 0 {
            return result;
        }

        // find shortest word
        let target_word = strs.iter().min().unwrap().chars();

        //iterate shortest word
        for (index, current_char) in target_word.enumerate() {
            //loop all world
            for word in &strs {
                //if one word have diffrent char at current index loop return result
                if word.chars().collect::<Vec<char>>()[index] != current_char {
                    return result;
                }
            }
            // all word have same char so program add it to result string
            result.push(current_char);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn solution_1_test_1() {
        let input: Vec<String> = vec!["flower", "flow", "flight"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        let res = Solution::longest_common_prefix(input);
        let exptected = "fl".to_string();
        assert_eq!(res, exptected);
    }

    #[test]
    fn solution_1_test_2() {
        let input: Vec<String> = vec!["dog", "racecar", "car"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        let res = Solution::longest_common_prefix(input);
        let exptected = "".to_string();
        assert_eq!(res, exptected);
    }
}
