mod reverse_vowels_of_a_string;
mod two_sum;
fn main() {
    let nums: Vec<i32> = vec![2, 7, 11, 15];
    let target: i32 = 9;
    let a = two_sum::Solution::solution_2(nums, target);
    dbg!(a);
}
