// https://leetcode.com/problems/maximal-rectangle/

// 4 int for the corners
// and a cur_max_area

// https://leetcode.com/problems/maximal-rectangle/discuss/29054/Share-my-DP-solution
//
// for each row:
//   h[i] stores cur valid height on col i
//   lhs[i] stores the leftmost index k, that h[k] >= h[i]
//   rhs[i] stores the rightmost index k, that h[k] >= h[i]
//
// area = h[i] * (rhs[i] - lhs[i] + 1)

pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
    if matrix.is_empty() || matrix[0].is_empty() {
        return 0;
    }
    let m = matrix.len();
    let n = matrix[0].len();

    // we cache previous row height so we
    // don't iterate up and down on each matrix[i][j]
    let mut h: Vec<usize> = vec![0; n];
    let mut rhs: Vec<usize> = vec![n - 1; n];
    let mut lhs: Vec<usize> = vec![0; n];
    let mut cur_area: i32 = 0;

    // consider only one row
    // we can have height considered later
    for i in 0..m {
        // find right boundary
        let mut rb = (n - 1) as i32;
        for j in (0..n).rev() {
            if matrix[i][j] == '1' {
                rhs[j] = rhs[j].min(rb as usize);
            } else {
                rhs[j] = n - 1;
                rb = j as i32 - 1;
            }
        }
        // find left boundary
        let mut lb = 0;
        for j in 0..n {
            if matrix[i][j] == '1' {
                lhs[j] = lhs[j].max(lb);
            } else {
                lhs[j] = 0;
                lb = j + 1;
            }
        }
        // update height
        for j in 0..n {
            if matrix[i][j] == '1' {
                h[j] += 1;
                cur_area = cur_area.max((h[j] * (rhs[j] - lhs[j] + 1)) as i32);
            } else {
                h[j] = 0;
            }
        }
    }
    cur_area
}

#[test]
fn run() {
    let matrix = vec![vec!['1', '0', '1', '1']];
    assert_eq!(maximal_rectangle(matrix), 2);

    let matrix = vec![vec!['1', '0', '1', '1'], vec!['1', '0', '1', '1']];
    assert_eq!(maximal_rectangle(matrix), 4);

    let matrix = vec![
        vec!['1', '0', '1', '1'],
        vec!['1', '0', '1', '1'],
        vec!['0', '1', '1', '1'],
    ];
    assert_eq!(maximal_rectangle(matrix), 6);

    let matrix = vec![
        vec!['0', '1', '1', '0', '1'],
        vec!['1', '1', '0', '1', '0'],
        vec!['0', '1', '1', '1', '0'],
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '1', '1', '1'],
        vec!['0', '0', '0', '0', '0'],
    ];
    assert_eq!(maximal_rectangle(matrix), 9);
}
