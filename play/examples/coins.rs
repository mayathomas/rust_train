// 给定不同面额的硬币和一个总金额。编写一个函数来计算可以凑成总金额所需的最少的硬币个数。如果没有任何一种硬币组合能组成总金额，返回 -1。
// 你可以认为每种硬币的数量是无限的。
// 示例
// 示例 1：
// 输入：coins = [1, 2, 5], amount = 11
// 输出：3
// 解释：11 = 5 + 5 + 1
// 示例 2：
// 输入：coins = [2], amount = 3
// 输出：-1
// 解释：无法用面额为2的硬币凑成总金额3
// 示例 3：
// 输入：coins = [1], amount = 0
// 输出：0
// 解释：不需要任何硬币

fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    // 创建动态规划数组，初始值设为amount+1（一个无法达到的值，因为最多只需要amount个面值为1的硬币）
    let mut dp = vec![amount + 1; (amount + 1) as usize];
    // 基础情况：凑成金额0需要0个硬币
    dp[0] = 0;

    // 自底向上计算每个金额所需的最少硬币数
    for i in 1..=amount {
        // 尝试每种硬币面额
        for &coin in &coins {
            // 只有当当前金额i大于等于硬币面额时才能使用该硬币
            if i - coin >= 0 {
                // 打印当前状态，用于调试
                println!("dp[{i}]:{}, dp[{i}-{coin}]:{}", dp[i as usize], dp[(i - coin) as usize] + 1);
                // 状态转移方程：dp[i] = min(dp[i], dp[i-coin] + 1)
                // 即：当前金额的最少硬币数 = min(当前值, 使用当前硬币后剩余金额的最少硬币数+1)
                dp[i as usize] = dp[i as usize].min(dp[(i - coin) as usize] + 1);
            }
        }
    }

    // 如果最终结果仍为初始值，说明无法凑成目标金额，返回-1
    if dp[amount as usize] == amount + 1 {
        -1
    } else {
        // 否则返回所需的最少硬币数
        dp[amount as usize]
    }
}

fn main(){
    let coins = vec![2, 3, 5];
    let amount = 13;
    assert_eq!(coin_change(coins, amount), 3);
}

/**
 * 测试零钱兑换函数
 * 
 * 示例：硬币面额为[1,2,5]，目标金额为11
 * 最优解为：5+5+1=11，共需3个硬币
 */
#[test]
fn test_coin_change() {
    let coins = vec![2, 3, 5];
    let amount = 13;
    assert_eq!(coin_change(coins, amount), 3);
}
