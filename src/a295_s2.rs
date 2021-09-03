// https://leetcode.com/problems/find-median-from-data-stream/

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct MedianFinder {
    low: BinaryHeap<i32>,
    high: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    pub fn new() -> Self {
        Self {
            low: BinaryHeap::with_capacity(25000),
            high: BinaryHeap::with_capacity(25000),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        match (self.low.peek(), self.high.peek()) {
            (None, Some(_)) => self.high.push(Reverse(num)),
            (None, None) => self.low.push(num),

            (Some(_), Some(Reverse(h))) if num >= *h => self.high.push(Reverse(num)),
            (Some(_), Some(_)) => self.low.push(num),
            (Some(_), None) => self.low.push(num),
        };

        let lh = self.low.len();
        let rh = self.high.len();

        if lh > rh {
            if let Some(l) = self.low.pop() {
                self.high.push(Reverse(l));
            }
        } else if lh < rh {
            if let Some(Reverse(h)) = self.high.pop() {
                self.low.push(h);
            }
        }
    }

    pub fn find_median(&self) -> f64 {
        let lh = self.low.len();
        let rh = self.high.len();

        match (self.low.peek(), self.high.peek()) {
            (Some(left), Some(Reverse(right))) => {
                if (lh + rh) % 2 == 0 {
                    *left as f64 * 0.5 + *right as f64 * 0.5
                } else if lh > rh {
                    *left as f64
                } else {
                    *right as f64
                }
            }
            (Some(l), None) => *l as f64,
            (None, Some(Reverse(h))) => *h as f64,
            (None, None) => 0.0,
        }
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
