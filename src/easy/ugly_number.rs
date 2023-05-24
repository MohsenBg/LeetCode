#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn is_ugly(n: i32) -> bool {
        // prime factors are limited to 2, 3, and 5
        let base_prim: [i32; 3] = [2, 3, 5];

        //mut n
        let mut n = n;
        loop {
            // use this var to check if n changed or not
            // if n not changed it's means n isn't divisible by 2, 3 and 5
            let mut n_changed = false;

            for num in base_prim {
                //this loop check if n has remainder when divided to ,2 ,3 ,5
                if n % num == 0 {
                    // and than divided
                    n /= num;
                    // set n_changed var true beacuse n changed

                    n_changed = true;
                }
            }

            // break loop if our number smaller than 2 beacuse 1 and zero and divided any more
            // if n not changed it's means n isn't divisible by 2, 3 and 5
            if n <= 1 || !n_changed {
                break;
            }
        }

        n == 1
    }
}

#[cfg(test)]

mod tests {
    use super::Solution;
    #[test]
    fn solution_1_test_1() {
        assert_eq!(Solution::is_ugly(6), true);
    }

    #[test]
    fn solution_1_test_2() {
        assert_eq!(Solution::is_ugly(1), true);
    }

    #[test]
    fn solution_1_test_3() {
        assert_eq!(Solution::is_ugly(14), false);
    }
}
