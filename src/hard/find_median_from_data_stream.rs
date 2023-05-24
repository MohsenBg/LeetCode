#[allow(dead_code)]
struct MedianFinder {
    val: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    #[allow(dead_code)]
    fn new() -> Self {
        MedianFinder { val: Vec::new() }
    }

    #[allow(dead_code)]
    fn add_num(&mut self, num: i32) {
        self.val.push(num);
    }

    #[allow(dead_code)]
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
    fn solution_1_test_1() {
        let mut median_finder: MedianFinder = MedianFinder::new();
        median_finder.add_num(1); // arr = [1]
        median_finder.add_num(2); // arr = [1, 2]
        median_finder.find_median(); // return 1.5 (i.e., (1 + 2) / 2)
        median_finder.add_num(3); // arr[1, 2, 3]
        let mid = median_finder.find_median(); // return 2.0
        assert_eq!(mid, 2.0);
    }
}
