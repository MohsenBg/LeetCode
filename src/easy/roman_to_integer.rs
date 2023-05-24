use std::collections::HashMap;
#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn roman_to_int(s: String) -> i32 {
        let table: HashMap<char, i32> = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);

        let mut sum: i32 = 0;
        let chars: Vec<char> = s.chars().collect();
        let mut index: usize = 0;
        while index < chars.len() {
            // current_char in loop
            let current_char = chars[index];
            if index != chars.len() - 1 {
                // next char in loop
                let next_char = chars[index + 1];

                // check if next char is bigger than current char
                // Example
                // VI  => I > V => False  then V     => sum +=5      => index +=1
                // IV  => V > I => True   then V - I => sum += 5 - 1 => index+=2
                if table[&next_char] > table[&current_char] {
                    sum += table[&next_char] - table[&current_char];
                    index += 2;
                    continue;
                }
            }
            sum += table[&current_char];
            index += 1;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn solution_1_test_1() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3)
    }

    #[test]
    fn solution_1_test_2() {
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58)
    }
    #[test]

    fn solution_1_test_3() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994)
    }
}
