// 给你一个整数 n 表示数轴上的房屋数量，编号从 0 到 n - 1 。
// 另给你一个二维整数数组 offers ，其中 offers[i] = [starti, endi, goldi] 表示第 i 个买家想要以 goldi 枚金币的价格购买从 starti 到 endi 的所有房屋。
// 作为一名销售，你需要有策略地选择并销售房屋使自己的收入最大化。
// 返回你可以赚取的金币的最大数目。
// 注意 同一所房屋不能卖给不同的买家，并且允许保留一些房屋不进行出售。

// 如果第n-1个房子不卖，则f(n-1) = f(n-2)
// 如果第n-1个房子卖，则f(n-1) = f(start-1) + gold
pub fn maximize_the_profit(n: i32, offers: Vec<Vec<i32>>) -> i32 {
    let mut group = vec![vec![]; n as usize];
    for offer in offers.iter() {
        group[offer[1] as usize].push((offer[0], offer[2]));
    }

    let mut f = vec![0; (n+1) as usize];
    for (end, the_group) in group.iter().enumerate() {
        f[end + 1] = f[end];
        for (start, gold) in the_group.iter() {
            f[end + 1] = f[end + 1].max(f[*start as usize] + gold);
        }
    }

    f[n as usize]
}

fn main() {
    let a = vec![vec![0,0,1],vec![0,2,10],vec![1,3,2]];
    let a = maximize_the_profit(5, a);
    println!("{}", a);
}
