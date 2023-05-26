#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut alphabet_storage: [u32; 26] = [0; 26];

        for ch in magazine.chars() {
            let index = (ch as u8 - b'a') as usize;
            alphabet_storage[index] += 1;
        }

        for ch in ransom_note.chars().into_iter() {
            let index = (ch as u8 - b'a') as usize;
            if alphabet_storage[index] == 0 {
                return false;
            }
            alphabet_storage[index] -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::Solution;
    #[test]
    pub fn solution_1_test_1() {
        let input = ["a", "b"];
        let expected = false;
        let output = Solution::can_construct(input[0].to_string(), input[1].to_string());
        assert_eq!(expected, output);
    }

    #[test]
    pub fn solution_1_test_2() {
        let input = ["aa", "ab"];
        let expected = false;
        let output = Solution::can_construct(input[0].to_string(), input[1].to_string());
        assert_eq!(expected, output);
    }

    #[test]
    pub fn solution_1_test_3() {
        let input = ["aa", "aab"];
        let expected = true;
        let output = Solution::can_construct(input[0].to_string(), input[1].to_string());
        assert_eq!(expected, output);
    }
}
