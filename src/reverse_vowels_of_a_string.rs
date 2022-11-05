#[derive(Debug)]
pub struct Solution {}
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut str_vec: Vec<char> = s.chars().into_iter().collect();
        let mut vewels_chars = Vec::new();

        for c in str_vec.clone() {
            if is_char_vowels(c) {
                vewels_chars.push(c);
            }
        }

        dbg!(vewels_chars.clone());
        for (index, value) in str_vec.clone().into_iter().enumerate() {
            if is_char_vowels(value) {
                let new_char = vewels_chars.pop().unwrap();
                str_vec[index] = new_char;
            }
        }

        return str_vec.into_iter().collect();
    }
}

fn is_char_vowels(c: char) -> bool {
    let c = c.to_ascii_lowercase();
    let vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    for vowel in vowels {
        if vowel == c {
            return true;
        }
    }
    return false;
}
