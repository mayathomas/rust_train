pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
    // 每个位置在剩余油量下的路径数
    let mut f = vec![vec![-1; fuel as usize + 1]; locations.len()];

    fn dfs(locations: &Vec<i32>, pos: i32, finish: i32, fuel: i32, f: &mut Vec<Vec<i32>>) -> i32 {
        let pos = pos as usize;
        let fuel = fuel as usize;
        let finish = finish as usize;
        if f[pos][fuel] != -1 {
            return f[pos][fuel];
        }

        f[pos][fuel] = 0;
        // 如果一步到达不了，则永远到达不了
        if fuel < (locations[pos] - locations[finish]).abs() as usize {
            return 0;
        }

        for (i, &loc) in locations.iter().enumerate() {
            if i != pos {
                let need = (loc - locations[pos]).abs();
                if fuel >= need as usize {
                    f[pos][fuel] += dfs(locations, i as i32, finish as i32, fuel as i32 - need, f);
                    f[pos][fuel] %= 1000000007;
                }
            }
        }
        // 如果当前位置就在终点，算是一种路径
        if pos == finish {
            f[pos as usize][fuel as usize] += 1;
        }
        f[pos as usize][fuel as usize] % 1000000007
    }
    dfs(&locations, start, finish, fuel, &mut f) % 1000000007
}

// dp[start][cur_fuel] += dp[j][cur_fuel - need]
pub fn count_routes_dp(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
    let n = locations.len();
    let mut dp = vec![vec![0; (fuel as usize) + 1]; n];

    for i in 0..=fuel {
        dp[finish as usize][i as usize] = 1;
    }

    for cur_fuel in 0..=fuel {
        for j in 0..n {
            for k in 0..n {
                if j != k {
                    let need = (locations[j] - locations[k]).abs();
                    if cur_fuel >= need {
                        dp[j][cur_fuel as usize] += dp[k][(cur_fuel - need) as usize];
                        dp[j][cur_fuel as usize] %= 1000000007;
                    }
                }
            }
        }
    }
    dp[start as usize][fuel as usize]
}

fn main() {
    let locations = vec![2, 3, 6, 8, 4];
    let start = 1;
    let finish = 3;
    let fuel = 5;
    println!("{}", count_routes(locations, start, finish, fuel));
}
