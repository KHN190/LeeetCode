// https://leetcode.com/problems/median-of-two-sorted-arrays/

// Median of Two Sorted Arrays

// Given two sorted arrays nums1 and nums2 of size m and n respectively,
// return the median of the two sorted arrays.

// The overall run time complexity should be O(log (m+n)).

pub struct MedianFinder {
    window: Vec<i32>,
}

impl MedianFinder {
    pub fn new() -> Self {
        Self {
            window: Vec::with_capacity(2000),
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

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    // merge 2 lists, return median
    // same as find median from a stream
    let (mut c1, mut c2) = (0usize, 0usize);
    let l1 = nums1.len();
    let l2 = nums2.len();
    let mut obj = MedianFinder::new();

    loop {
        // if nums1 empty, merge nums2 to window
        if c1 == l1 {
            for i in c2..l2 {
                obj.add_num(nums2[i]);
            }
            break;
        }
        // if nums2 empty, merge nums1 to window
        if c2 == l2 {
            for i in c1..l1 {
                obj.add_num(nums1[i]);
            }
            break;
        }
        // else, find smaller and advance
        let v1 = nums1[c1];
        let v2 = nums2[c2];
        if v1 < v2 {
            obj.add_num(v1);
            c1 += 1;
        } else {
            obj.add_num(v2);
            c2 += 1;
        }
    }

    obj.find_median()
}

#[test]
fn run() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.0);

    let nums1 = vec![1, 2];
    let nums2 = vec![3, 4];
    assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.5);
}
