struct MedianFinder {
    val: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        MedianFinder { val: Vec::new() }
    }

    fn add_num(&mut self, num: i32) {
        self.val.push(num);
    }

    fn find_median(&mut self) -> f64 {
        if self.val.len() == 0 {
            return 0.0;
        }

        self.val.sort();
        let mid = self.val.len() / 2;
        println!(" mid:{} \n len:{}", mid, self.val.len());
        if self.val.len() % 2 == 0 {
            (self.val[mid - 1] + self.val[mid]) as f64 / 2.0
        } else {
            self.val[mid] as f64
        }
    }
}

//
// Your MedianFinder object will be instantiated and called as such:
// let obj = MedianFinder::new();
// obj.add_num(num);
// let ret_2: f64 = obj.find_median();
//

#[cfg(test)]
mod tests {
    use super::MedianFinder;

    #[test]
    fn Solution_1_test_1() {
        let mut medianFinder: MedianFinder = MedianFinder::new();
        medianFinder.add_num(1); // arr = [1]
        medianFinder.add_num(2); // arr = [1, 2]
        medianFinder.find_median(); // return 1.5 (i.e., (1 + 2) / 2)
        medianFinder.add_num(3); // arr[1, 2, 3]
        let mid = medianFinder.find_median(); // return 2.0
        assert_eq!(mid, 2.0);
    }
}
