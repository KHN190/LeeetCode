// https://leetcode.com/problems/max-area-of-island/

// find the largest connected set
// see als: LC#200

pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
    let row = grid.len();
    let col = grid[0].len();
    let mut res = 0;

    for i in 0..row {
        for j in 0..col {
            res = res.max(dfs(i, j, &mut grid));
        }
    }
    res
}

fn dfs(i: usize, j: usize, grid: &mut Vec<Vec<i32>>) -> i32 {
    // if is visted or 0
    if grid[i][j] <= 0 {
        return 0;
    }
    // mark as visited
    grid[i][j] = -1;

    let row = grid.len();
    let col = grid[0].len();

    // visit 4 directions
    let mut area = 1;
    if i >= 1 {
        area += dfs(i - 1, j, grid);
    }
    if j >= 1 {
        area += dfs(i, j - 1, grid);
    }
    if j + 1 < col {
        area += dfs(i, j + 1, grid);
    }
    if i + 1 < row {
        area += dfs(i + 1, j, grid);
    }
    area
}

#[test]
fn run() {
    let grid: Vec<Vec<i32>> = vec![
        vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
    ];
    let n = max_area_of_island(grid);
    assert_eq!(n, 6);
}
