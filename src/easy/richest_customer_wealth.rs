#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut wealths: Vec<i32> = vec![];
        for account in accounts.iter() {
            let mut total_wealth: i32 = 0;
            for wealth in account.iter() {
                total_wealth += wealth;
            }
            wealths.push(total_wealth)
        }

        wealths.iter().max().unwrap().clone()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn solution_1_test_1() {
        let input = vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]];
        let output = Solution::maximum_wealth(input);
        let expected = 17;
        assert_eq!(expected, output);
    }
}
