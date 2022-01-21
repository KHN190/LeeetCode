// rolling hash (robin karp) solution

// https://en.wikipedia.org/wiki/Rabin–Karp_algorithm

// first we match the hash, then we trivially compare
// if the segment match if hash matches.

// @todo review this question and understand it

const MOD: u128 = 1001;
const BASE: u128 = 2;

pub fn find_length(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let mut res = 0;
    let (mut lhs, mut rhs) = (0, a.len().min(b.len()));

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

    // do trivial check in case of hash collision
    // hashie = hash(i..win_size + i)
    for (i, hashie) in ha.iter().enumerate() {
        if hb.contains(&hashie) {
            let j = hb.iter().position(|e| e == hashie).unwrap();
            if trivial_check(a, b, i, j, win_size) {
                return true;
            }
        }
    }
    false
}

// check char match from (i..win_size + i)
fn trivial_check(a: &Vec<i32>, b: &Vec<i32>, i: usize, j: usize, win_size: usize) -> bool {
    a[i..i + win_size] == b[j..j + win_size]
}

// calcualte all hashies for 0..win_size
fn rolling_hash(nums: &Vec<i32>, win_size: usize) -> Vec<i32> {
    let mut res = vec![];
    let mut hashie = 0;
    // when i < win_size, rolling hash is calculated by simply
    // appending data[i] to the end.
    for i in 0..win_size {
        hashie = (hashie * BASE + nums[i] as u128) % MOD;
    }
    res.push(hashie as i32);
    // when we keep win_size, we deduct the head hash data[i - win_size],
    // then append data[i] to the end.
    let (pow, _) = BASE.overflowing_pow(win_size as u32 - 1);

    for i in win_size..nums.len() {
        // @warn keep base small so we don't have overflow by power
        let last_hashie = nums[i - win_size] as u128 * pow % MOD;
        // for sanity, hashie must >= 0 after minus
        if last_hashie > hashie {
            hashie += MOD;
        }
        hashie -= last_hashie;
        hashie = (hashie * BASE + nums[i] as u128) % MOD;
        res.push(hashie as i32);
    }
    // we have rolling hash from 0..nums.len()
    res
}

#[test]
fn run() {
    let n1: Vec<i32> = vec![1, 0, 1, 0, 0, 0, 0, 0, 1, 1];
    let n2: Vec<i32> = vec![1, 1, 0, 1, 1, 0, 0, 0, 0, 0];
    assert_eq!(find_length(n1, n2), 6);

    let n1: Vec<i32> = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let n2: Vec<i32> = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
    ];
    assert_eq!(find_length(n1, n2), 59);

    let n1: Vec<i32> = vec![69, 51, 94, 52, 72, 74, 65, 65, 99, 1];
    let n2: Vec<i32> = vec![65, 99, 82, 27, 43, 95, 9, 20, 13, 99];
    assert_eq!(find_length(n1, n2), 2);

    let n1: Vec<i32> = vec![70, 39, 25, 40, 7];
    let n2: Vec<i32> = vec![52, 20, 67, 5, 31];
    assert_eq!(find_length(n1, n2), 0);
}
