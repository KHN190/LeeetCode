// https://leetcode.com/problems/find-median-from-data-stream/

// FollowUp: run-length compress

pub(crate) struct MedianFinder {
    nums: Vec::<i32>,
    freq: Vec::<usize>,
}

#[allow(dead_code)]
impl MedianFinder {
    pub fn new() -> Self {
        Self {
            nums: Vec::new(),
            freq: Vec::new()
        }
    }

    fn len(&self) -> usize {
        self.nums.len()
    }

    pub fn add_num(&mut self, num: i32) {
        // find where to insert
        match self.nums.binary_search(&num) {
            Ok(x) => {
                // increase freq
                self.freq[x] += 1;
            },
            Err(x) => {
                // add new num
                self.nums.insert(x, num);
                self.freq.insert(x, 1);
            }
        };
    }

    pub fn find_median(&self) -> f64 {
        // get median
        // @warn index calculation!
        let len: usize = self.freq.iter().sum();
        let mid = len / 2;

        // median
        let mut median = 0.0;
        if len % 2 == 1 {
            // mid
            let mut cnt = 0;
            for (i, x) in self.freq.iter().enumerate() {
                cnt += x;
                if cnt > mid {
                    median = self.nums[i] as f64;
                    break;
                }
            }
        } else {
            // mid, mid - 1
            let mut cnt = 0;
            let mut n1 = None;
            let mut n2 = None;
            for (i, x) in self.freq.iter().enumerate() {
                cnt += x;
                if n1 == None && cnt > mid - 1 {
                    n1 = Some(self.nums[i] as f64);
                }
                if n2 == None && cnt > mid {
                    n2 = Some(self.nums[i] as f64);
                }
                if n1 != None && n2 != None { break; }
            };
            median = n1.unwrap() * 0.5 + n2.unwrap() * 0.5;
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

#[test]
fn run_large_data() {
    use jemalloc_ctl::{ epoch, stats };
    let e = epoch::mib().unwrap();
    let allocated = stats::allocated::mib().unwrap();

    // init memory
    let m0: f64 = allocated.read().unwrap() as f64 / 1000.0;
    e.advance().unwrap();

    // memory after o1 finder
    let mut o1 = MedianFinder::new();
    for _ in 0..100000 {
        o1.add_num(10);
    }
    let m1: f64 = allocated.read().unwrap() as f64 / 1000.0;
    e.advance().unwrap();

    // memory after o2 finder
    let mut o2 = crate::a295::MedianFinder::new();
    for _ in 0..100000 {
        o2.add_num(10);
    }
    let m2: f64 = allocated.read().unwrap() as f64 / 1000.0;
    e.advance().unwrap();

    assert_eq!(o1.find_median(), 10.0);
    assert_eq!(o1.len(), 1);

    assert_eq!(o2.find_median(), 10.0);
    assert_eq!(o2.len(), 100000);

    // this solution uses much less memory
    assert!((m2 - m1) > (m1 - m0) * 1000.0);
}
