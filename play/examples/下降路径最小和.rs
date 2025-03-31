// f[i][j] = min(f[i-1][j], f[i-1][j-1], f[i-1][j+1]) + matrix[i][j]
pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
    let mut f = matrix.clone();
    for i in 1..matrix.len() {
        for j in 0..matrix[i].len() {
            if j == 0 {
                f[i][j] = f[i - 1][j].min(f[i - 1][j + 1]) + matrix[i][j];
            } else if j == matrix[i].len() - 1 {
                f[i][j] = f[i - 1][j].min(f[i - 1][j - 1]) + matrix[i][j];
            } else {
                f[i][j] = f[i - 1][j].min(f[i - 1][j - 1]).min(f[i - 1][j + 1]) + matrix[i][j];
            }
        }
    }
    f[matrix.len() - 1].iter().min().unwrap().clone()
}

fn main() {
    let matrix = vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]];
    println!("{}", min_falling_path_sum(matrix));
}
