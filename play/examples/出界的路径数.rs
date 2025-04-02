// dp[move][i][j] = dp[move-1][i+1][j] + dp[move-1][i-1][j] + dp[move-1][i][j+1] + dp[move-1][i][j-1]
pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
    let i = m as usize;
    let j = n as usize;
    let max_move = max_move as usize;
    let mut res = 0;
    let mut dp = vec![vec![vec![0; j]; i]; max_move + 1];
    let directions = vec![vec![0, 1], vec![1, 0], vec![0, -1], vec![-1, 0]];
    dp[0][start_row as usize][start_column as usize] = 1;
    for i in 0..max_move {
        for j in 0..m {
            for k in 0..n {
                let count = dp[i as usize][j as usize][k as usize];
                for d in directions.iter() {
                    let new_i = j + d[0];
                    let new_j = k + d[1];
                    if new_i >= 0 && new_i < m && new_j >= 0 && new_j < n {
                        dp[i + 1][new_i as usize][new_j as usize] =
                            (dp[i + 1][new_i as usize][new_j as usize] + count) % 1000000007;
                    } else {
                        res = (res + count) % 1000000007;
                    }
                }
            }
        }
    }
    res
}

fn main() {}
