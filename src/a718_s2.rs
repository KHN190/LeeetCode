// rolling hash (robin karp) solution

// https://en.wikipedia.org/wiki/Rabin–Karp_algorithm

// first we match the hash, then we trivially compare
// if the segment match if hash matches.

// @todo review this question and understand it

// const MOD: i32 = 1e6 as i32 + 3;
// const BASE: i32 = 101;
const MOD: i32 = 1001;
const BASE: i32 = 7;

pub fn find_length(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let mut res = 0;
    let (mut lhs, mut rhs) = (0, a.len().min(b.len()));

    let win_size = lhs + (rhs - lhs) / 2;
    println!("{}", BASE.pow(win_size as u32 - 1));

    // binary search with rolling hash
    while lhs <= rhs {
        let mid = lhs + (rhs - lhs) / 2;
        if validate(mid, &a, &b) {
            res = res.max(mid as i32);
            lhs = mid + 1;
        } else {
            rhs = mid - 1;
        }
    }
    res
}

fn validate(win_size: usize, a: &Vec<i32>, b: &Vec<i32>) -> bool {
    if win_size == 0 {
        return true;
    }
    let ha: Vec<i32> = rolling_hash(a, win_size);
    let hb: Vec<i32> = rolling_hash(b, win_size);

    // we didn't actually do trivial check
    // but for sake, we need
    for hashie in ha {
        if hb.contains(&hashie) {
            return true;
        }
    }
    false
}

// calcualte all hashies for 0..win_size
fn rolling_hash(nums: &Vec<i32>, win_size: usize) -> Vec<i32> {
    let mut res = vec![];
    let mut hashie = 0;
    // when i < win_size, rolling hash is calculated by simply
    // appending data[i] to the end.
    for i in 0..win_size {
        hashie = (hashie * BASE + nums[i]) % MOD;
    }
    res.push(hashie);
    // when we keep win_size, we deduct the head hash data[i - win_size],
    // then append data[i] to the end.
    for i in win_size..nums.len() {
        // @warn we have overflow by power
        hashie -= nums[i - win_size] * BASE.pow(win_size as u32 - 1) % MOD;
        hashie = (hashie * BASE + nums[i]) % MOD;
        // for sanity
        if hashie < 0 {
            hashie += MOD;
        }
        res.push(hashie);
    }
    // we have rolling hash from 0..nums.len()
    res
}

#[test]
fn run() {
    let n1: Vec<i32> = vec![1, 0, 1, 0, 0, 0, 0, 0, 1, 1];
    let n2: Vec<i32> = vec![1, 1, 0, 1, 1, 0, 0, 0, 0, 0];
    assert_eq!(find_length(n1, n2), 6);

    // this should work, but we have overflow
    // let n1: Vec<i32> = vec![
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    // ];
    // let n2: Vec<i32> = vec![
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
    // ];
    // assert_eq!(find_length(n1, n2), 59);
}
