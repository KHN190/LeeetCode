// https://leetcode.com/problems/number-of-islands/

// union find

use crate::types::UnionFind;

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let row = grid.len();
    let col = grid[0].len();
    let n_total = row * col;
    // all elem are created with themselves being their parents
    let mut uf = UnionFind::new(n_total);
    // create connections
    for i in 0..row {
        for j in 0..col {
            // if is '1' then go 2 directions and union them
            if grid[i][j] == '1' {
                // curr id
                let n0 = i * col + j;
                // right neighbour is connected
                if j + 1 < col && grid[i][j + 1] == '1' {
                    uf.union(n0, i * col + j + 1);
                }
                // down neighbour is connected
                if i + 1 < row && grid[i + 1][j] == '1' {
                    uf.union(n0, (i + 1) * col + j);
                }
            }
        }
    }
    // count the isolated islands, they have themselves as parent
    let mut res = 0;
    for i in 0..row {
        for j in 0..col {
            let n0 = i * col + j;
            if grid[i][j] == '1' && uf.find(n0) == n0 {
                res += 1;
            }
        }
    }
    res
}

#[test]
fn run() {
    let grid: Vec<Vec<char>> = vec![
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['0', '0', '0', '0', '1'],
    ];
    assert_eq!(num_islands(grid), 3);

    let grid: Vec<Vec<char>> = vec![
        vec!['0', '1', '0'],
        vec!['1', '0', '1'],
        vec!['0', '1', '0'],
    ];
    assert_eq!(num_islands(grid), 4);
}
