pub struct Solution {}

impl Solution {
    pub fn compute_area(
        ax1: i32,
        ay1: i32,
        ax2: i32,
        ay2: i32,
        bx1: i32,
        by1: i32,
        bx2: i32,
        by2: i32,
    ) -> i32 {
        let area1 = i32::abs(ax2 - ax1) * i32::abs(ay2 - ay1);
        let area2 = i32::abs(bx2 - bx1) * i32::abs(by2 - by1);
        let cx = i32::min(ax2, bx2) - i32::max(ax1, bx1);
        let cy = i32::min(ay2, by2) - i32::max(ay1, by1);
        let mut area3 = 0;
        if cx > 0 && cy > 0 {
            area3 = cx * cy
        }
        area1 + area2 - area3
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solution_1_test_1() {
        let res = Solution::compute_area(-3, 0, 3, 4, 0, -1, 9, 2);
        assert_eq!(res, 45);
    }
    #[test]
    fn solution_1_test_2() {
        let res = Solution::compute_area(-2, -2, 2, 2, -2, -2, 2, 2);
        assert_eq!(res, 16);
    }
}
