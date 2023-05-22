pub struct Solution {}
impl Solution {
    pub fn sum(num1: i32, num2: i32) -> i32 {
        num1 + num2
    }
}

mod test {
    use super::Solution;

    #[test]
    fn solution_1_test_1() {
        let num1 = 12;
        let num2 = 5;
        let expected = 17;
        assert_eq!(expected, Solution::sum(num1, num2));
    }
    #[test]
    fn solution_1_test_2() {
        let num1 = -10;
        let num2 = 4;
        let expected = -6;
        assert_eq!(expected, Solution::sum(num1, num2));
    }
}
