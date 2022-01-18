use std::cmp::Ordering;

// select pivot and sort rhs, lhs >= rhs
fn partition_by_freq(slice: &mut [(i32, usize)], lhs: usize, rhs: usize) -> usize {
    // pivot is on the right, = median + lhs
    let pivot = lhs + (rhs - lhs) / 2;
    slice.swap(pivot, rhs);

    // sort the rhs nums
    let mut i = lhs;
    for j in lhs..rhs {
        if slice[j].1 > slice[rhs].1 {
            slice.swap(i, j);
            i += 1;
        }
    }

    slice.swap(i, rhs);
    i
}

// slice: (num, freq)
pub fn quick_select_by_freq(slice: &mut [(i32, usize)], mut lhs: usize, mut rhs: usize, k: usize) {
    while lhs < rhs {
        let mid = partition_by_freq(slice, lhs, rhs);

        match k.cmp(&mid) {
            Ordering::Greater => lhs = mid + 1,
            Ordering::Equal => rhs = mid,
            Ordering::Less => rhs = mid - 1,
        }
    }
}

// select pivot and sort rhs, lhs >= rhs
fn partition(slice: &mut [i32], lhs: usize, rhs: usize) -> usize {
    // pivot is on the right, = median + lhs
    let pivot = lhs + (rhs - lhs) / 2;
    slice.swap(pivot, rhs);

    // sort the rhs nums
    let mut i = lhs;
    for j in lhs..=rhs {
        if slice[j] > slice[rhs] {
            slice.swap(i, j);
            i += 1;
        }
    }

    slice.swap(i, rhs);
    i
}

pub fn quick_select(slice: &mut [i32], mut lhs: usize, mut rhs: usize, k: usize) {
    while lhs < rhs {
        let mid = partition(slice, lhs, rhs);

        match k.cmp(&mid) {
            Ordering::Greater => lhs = mid + 1,
            Ordering::Equal => rhs = mid,
            Ordering::Less => rhs = mid - 1,
        }
    }
}
