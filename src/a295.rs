// https://leetcode.com/problems/find-median-from-data-stream/

pub struct MedianFinder {
    window: Vec<i32>,
}

impl MedianFinder {
    pub fn new() -> Self {
        Self {
            window: Vec::with_capacity(50000),
        }
    }

    pub fn len(&self) -> usize {
        self.window.len()
    }

    pub fn add_num(&mut self, num: i32) {
        // find where to insert
        let idx = match self.window.binary_search(&num) {
            Ok(x) => x,
            Err(x) => x,
        };
        // add new elem
        self.window.insert(idx, num);
    }

    pub fn find_median(&self) -> f64 {
        // get median
        // @warn index calculation!
        let len = self.window.len();
        let mid = len / 2;
        let median = if len % 2 == 1 {
            self.window[mid] as f64
        } else {
            self.window[mid - 1] as f64 * 0.5 + self.window[mid] as f64 * 0.5
        };
        median
    }
}

#[test]
fn run() {
    let mut obj = MedianFinder::new();

    obj.add_num(2);
    assert_eq!(obj.find_median(), 2.0);

    obj.add_num(1);
    assert_eq!(obj.find_median(), 1.5);

    obj.add_num(3);
    assert_eq!(obj.find_median(), 2.0);
}
