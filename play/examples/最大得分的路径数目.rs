use std::vec;

// f[i][j] = max(f[i+1][j+1],f[i+1][j]+f[i][j+1]) + f[i][j]
pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
    let board = board
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut f = board
        .iter()
        .map(|s| {
            s.iter()
                .map(|&c| match c {
                    'X' => (-1,-1),
                    'E' => (0, 1),
                    'S' => (0, 1),
                    _ => (c.to_digit(10).unwrap() as i32, 0),
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .collect::<Vec<Vec<(i32, i32)>>>();

    for i in (0..board.len()).rev() {
        for j in (0..board[i].len()).rev() {
            if f[i][j].0 == -1 || (i == board.len() - 1 && j == board[i].len() - 1) {
                continue;
            } else if i == board.len() - 1 && j < board[i].len() - 1 {
                f[i][j] = (f[i][j].0 + f[i][j + 1].0, f[i][j + 1].1);
            } else if j == board[i].len() - 1 && i < board.len() - 1 {
                f[i][j] = (f[i][j].0 + f[i + 1][j].0, f[i + 1][j].1);
            } else {
                let v = Vec::<(i32, i32)>::from([f[i + 1][j + 1], f[i + 1][j], f[i][j + 1]]);
                let max = match v.iter().filter(|&&x| x.1 != -1).max() {
                    Some(x) => x,
                    None => &(-1, -1),
                };
                if max.0 == -1 {
                    f[i][j] = (-1, -1);
                } else {
                    let max_count = v.iter().filter(|&x| x.0 == max.0).map(|x| x.1).sum::<i32>();
                    f[i][j] = (max.0 + f[i][j].0, max_count % 1000000007);
                }
            }
        }
    }
    match f[0][0].0 {
        -1 => vec![0, 0],
        _ => vec![f[0][0].0, f[0][0].1],
    }
}

fn main() {
    // E32
    // 2X2
    // 12S
    // let board = vec!["E32".to_string(), "2X2".to_string(), "12S".to_string()];
    // E12
    // 1X1
    // 21S
    // let board = vec!["E12".to_string(), "1X1".to_string(), "21S".to_string()];
    // E11
    // XXX
    // 11S
    // let board = vec!["E11".to_string(), "XXX".to_string(), "11S".to_string()];
    // E5
    // XS
    // let board = vec!["E5".to_string(), "XS".to_string()];
    // ["E11345","X452XX","3X43X4","422812","284522","13422S"]
    // E11345
    // X452XX
    // 3X43X4
    // 422812
    // 284522
    // 13422S
    let board = vec!["E11345".to_string(), "X452XX".to_string(), "3X43X4".to_string(), "422812".to_string(), "284522".to_string(), "13422S".to_string()];
    println!("{:?}", paths_with_max_score(board));
}
