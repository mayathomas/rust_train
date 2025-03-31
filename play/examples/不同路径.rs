// f[i][j] = min(f[i-1][j], f[i][j-1]) + grid[i][j]
pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let mut f = vec![vec![0; grid[0].len()]; grid.len()];
    f[0][0] = grid[0][0];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if i > 0 && j > 0 {
                f[i][j] = f[i-1][j].min(f[i][j-1]) + grid[i][j];
            }
            if i > 0 && j == 0 {
                f[i][j] = f[i-1][j] + grid[i][j];
            }
            if j > 0 && i == 0 {
                f[i][j] = f[i][j-1] + grid[i][j];
            }
        }
    }
    f[grid.len() - 1][grid[0].len() - 1]
}

fn main() {
    let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
    println!("{}", min_path_sum(grid));
}
