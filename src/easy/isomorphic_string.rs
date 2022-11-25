pub struct Solution {}

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let t: Vec<char> = t.chars().collect();
        // map a -> b
        let mut map_char: HashMap<char, char> = HashMap::with_capacity(127);
        // map b used as value (store values of hashmap)
        let mut set_char: HashSet<char> = HashSet::with_capacity(127);

        //loop to all char in s
        for (index, c) in s.chars().into_iter().enumerate() {
            // check current char map to other char
            match map_char.get(&c) {
                //if not map to other char
                None => {
                    // check t char not used before as value
                    // if t char used before as value it's means other char map to this char
                    match set_char.get(&t[index]) {
                        Some(_) => return false,
                        // if t char not used before as value it's ok
                        None => {
                            // we can map new char to other char
                            // s char -> t char
                            map_char.insert(c, t[index]);
                            // make sure we add to set char
                            // we know in next loop it's map to other char
                            // if a -> b than we can use b beacuse b also b -> a
                            set_char.insert(t[index]);
                        }
                    }
                    map_char.insert(c, t[index]);
                }
                //map to other char
                Some(ch) => {
                    // if t char is not same as hashmap char return false
                    if ch != &t[index] {
                        return false;
                    }
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn solution_1_test_1() {
        let s = "egg".to_string();
        let t = "add".to_string();
        assert_eq!(Solution::is_isomorphic(s, t), true);
    }

    #[test]
    fn solution_1_test_2() {
        let s = "foo".to_string();
        let t = "bar".to_string();
        assert_eq!(Solution::is_isomorphic(s, t), false);
    }

    #[test]
    fn solution_1_test_3() {
        let s = "paper".to_string();
        let t = "title".to_string();
        assert_eq!(Solution::is_isomorphic(s, t), true);
    }

    #[test]
    fn solution_1_test_4() {
        let s = "badc".to_string();
        let t = "baba".to_string();
        assert_eq!(Solution::is_isomorphic(s, t), false);
    }
}
