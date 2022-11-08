pub struct Solution {}
impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        // convert num to vector of charecters
        let mut num: Vec<char> = num.to_string().chars().collect();

        // loop num chars with v and index
        for (i, v) in num.iter().enumerate() {
            // check if value is 6 then change it to 9 and break
            if v == &'6' {
                num[i] = '9';
                break;
            }
        }

        //convert char to string and then to i32
        num.into_iter().collect::<String>().parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn Solution_1_test_1() {
        let input = 9669;
        let expected = 9969;
        assert_eq!(Solution::maximum69_number(input), expected);
    }

    #[test]
    fn Solution_1_test_2() {
        let input = 9996;
        let expected = 9999;
        assert_eq!(Solution::maximum69_number(input), expected);
    }

    #[test]
    fn Solution_1_test_3() {
        let input = 9999;
        let expected = 9999;
        assert_eq!(Solution::maximum69_number(input), expected);
    }
}
