pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
    let mut f = matrix.clone();
    if matrix.len() == 0 {
        return 0;
    }
    // 只需要记录两个最小值的索引，不需要每次遍历j行获取最小值
    let mut i1 = -1;
    let mut i2 = -1;
    for (i, ele) in f[0].iter().enumerate() {
        if *ele < if i1 == -1 { i32::MAX } else { f[0][i1 as usize] } {
            i2 = i1;
            i1 = i as i32;
        } else if *ele < if i2 == -1 { i32::MAX } else { f[0][i2 as usize] } {
            i2 = i as i32;
        }
    }
    println!("i1={}, i2={}", i1, i2);
    for i in 1..matrix.len() {
        let mut ti1 = -1;
        let mut ti2 = -1;
        for j in 0..matrix[i].len() {
            f[i][j] = i32::MAX;
            // println!("==============");
            // println!("f[i][j]={}, ti1={}, ti2={}, i1={}, i2={}, i={}, j={}, val={}, f={:?}", f[i][j], ti1, ti2, i1, i2, i, j, matrix[i][j], f);
            let val = matrix[i][j];
            if j != i1 as usize {
                f[i][j] = f[i-1][i1 as usize] + val;
            } else {
                f[i][j] = f[i-1][i2 as usize] + val;
            }
            if f[i][j] < if ti1 == -1 { i32::MAX } else { f[i][ti1 as usize] } {
                ti2 = ti1;
                ti1 = j as i32;
            } else if f[i][j] < if ti2 == -1 { i32::MAX } else { f[i][ti2 as usize] } {
                ti2 = j as i32;
            }
            // println!("f[i][j]={}, ti1={}, ti2={}", f[i][j], ti1, ti2);
            // let min = (f[i-1][0..j].iter().chain(f[i-1][j+1..].iter())).min().unwrap();
            // f[i][j] = min + matrix[i][j];
        }
        i1 = ti1;
        i2 = ti2;
    }
    println!("{:?}", f);
    f[matrix.len() - 1].iter().min().unwrap().clone()
}

fn main() {
    let matrix = vec![vec![2,2,1,2,2], vec![2,2,1,2,2], vec![2,2,1,2,2], vec![2,2,1,2,2], vec![2,2,1,2,2]];
    println!("{}", min_falling_path_sum(matrix));
}
