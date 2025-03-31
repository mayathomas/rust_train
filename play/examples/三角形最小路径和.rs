// f[i][j] = min(f[i-1][j], f[i-1][j-1]) + triangle[i][j]
pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let mut f = triangle.clone();
    for i in 1..triangle.len() {
        for j in 0..triangle[i].len() {
            if j == 0{
                f[i][j] = f[i - 1][j] + triangle[i][j];
            } else if j == triangle[i].len() - 1 {
                f[i][j] = f[i - 1][j - 1] + triangle[i][j];
            } else {
                f[i][j] = f[i - 1][j].min(f[i - 1][j - 1]) + triangle[i][j];
            }
        }
    }
    // println!("{:?}", f);
    f[triangle.len() - 1].iter().min().unwrap().clone()
}

fn main() {
    let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
    println!("{}", minimum_total(triangle));
}
