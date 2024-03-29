use once_cell::sync::Lazy;
use std::{
    cell::RefCell,
    cmp::Ordering::{Equal, Greater, Less},
    rc::Rc,
};
#[allow(dead_code)]
pub struct Solution {}

//ignore this static variable it's for test
#[allow(dead_code)]
static mut TARGET: Lazy<Rc<RefCell<i32>>> = Lazy::new(|| Rc::new(RefCell::new(0 as i32)));

// ignore this function i use it for test
#[allow(dead_code)]
unsafe fn guess(num: i32) -> i32 {
    match TARGET.borrow().partial_cmp(&num).expect("") {
        Less => -1,
        Greater => 1,
        Equal => 0,
    }
}

// ignore this function i use it for test
#[allow(dead_code)]
unsafe fn change_target_number(new_target: i32) {
    let mut reference = (*(*TARGET)).borrow_mut();
    *reference = new_target;
}

impl Solution {
    #[allow(dead_code)]
    unsafe fn guess_number(n: i32) -> i32 {
        let mut half: i32 = n / 2;
        let mut new_guess: i32 = half;

        loop {
            half /= 2;
            match unsafe { guess(new_guess) } {
                -1 => {
                    new_guess -= half + 1;
                }
                1 => {
                    new_guess += half + 1;
                }
                _ => {
                    break;
                }
            }
            if new_guess < 0 {
                break;
            }
        }
        new_guess
    }
}

// these tests should run with single thread beacuse sometimes two thread want acsses to the static target
// and cause issus when chenged target value at the same time (the chance is small but it's good to prevent it)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solution_1_test_1() {
        unsafe { change_target_number(6) }
        let res = unsafe { Solution::guess_number(10) };
        assert_eq!(res, 6);
    }

    #[test]
    fn solution_1_test_2() {
        unsafe { change_target_number(1) }
        let res = unsafe { Solution::guess_number(1) };
        assert_eq!(res, 1);
    }

    #[test]
    fn solution_1_test_3() {
        unsafe { change_target_number(1) }
        let res = unsafe { Solution::guess_number(2) };
        assert_eq!(res, 1);
    }
}
