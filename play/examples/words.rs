/**
 * 计算两个字符串之间的编辑距离（Levenshtein距离）
 * 
 * 编辑距离是指将一个字符串转换成另一个字符串所需的最少操作次数
 * 允许的操作包括：插入一个字符、删除一个字符、替换一个字符
 * 
 * @param word1 第一个字符串
 * @param word2 第二个字符串
 * @return 两个字符串之间的编辑距离
 */
fn min_distance(word1: String, word2: String) -> i32 {
    // 获取两个字符串的长度
    let m = word1.len();
    let n = word2.len();
    // 创建动态规划表，dp[i][j]表示word1的前i个字符转换到word2的前j个字符所需的最少操作数
    let mut dp = vec![vec![0; n + 1]; m + 1];

    // 初始化边界条件：将空字符串转换为另一个字符串需要的操作数等于另一个字符串的长度
    // 如果word2是空字符串，那么只需要删除word1中的所有字符
    for i in 0..=m {
        dp[i][0] = i;
    }
    // 如果word1是空字符串，那么只需要插入word2中的所有字符
    for j in 0..=n {
        dp[0][j] = j;
    }

    // 填充动态规划表
    for i in 1..=m {
        for j in 1..=n {
            // 如果 word1[i-1] == word2[j-1]，则 dp[i][j] = dp[i-1][j-1]，因为最后一个字符相同，不需要额外操作
            // 判断当前字符是否相同，相同则不需要额外操作（cost=0），不同则需要替换（cost=1）
            println!("{} {} {}", &word1[i - 1..i], &word2[j - 1..j], &word1[i - 1..i] == &word2[j - 1..j]);
            let cost = if word1.as_bytes()[i - 1] == word2.as_bytes()[j - 1] { 0 } else { 1 };
            // 如果 word1[i-1] == word2[j-1]，则 dp[i][j] = dp[i-1][j-1]，因为最后一个字符相同，不需要额外操作。
            // 如果 word1[i-1] != word2[j-1]，则需要考虑三种操作：
            // 插入：dp[i][j] = dp[i][j-1] + 1
            // 删除：dp[i][j] = dp[i-1][j] + 1
            // 替换：dp[i][j] = dp[i-1][j-1] + 1
            // 综合考虑这三种操作，状态转移方程为: dp[i][j]=min(dp[i−1][j]+1,dp[i][j−1]+1,dp[i−1][j−1]+cost)
            // 取三种操作中的最小值
            dp[i][j] = (dp[i - 1][j] + 1)         // 删除操作
                .min(dp[i][j - 1] + 1)            // 插入操作
                .min(dp[i - 1][j - 1] + cost);    // 替换操作
        }
    }
    println!("{:?}", dp);

    // 返回最终的编辑距离
    dp[m][n] as i32
}

fn main(){
    let word1 = "horse".to_string();
    let word2 = "ros".to_string();
    println!("{}", min_distance(word1, word2));
}