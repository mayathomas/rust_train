// 一个专业的小偷，计划偷窃沿街的房屋。每间房内都藏有一定的现金，影响小偷偷窃的唯一制约因素就是相邻的房屋装有相互连通的防盗系统，如果两间相邻的房屋在同一晚上被小偷闯入，系统会自动报警。
// 给定一个代表每个房屋存放金额的非负整数数组 nums ，请计算 不触动警报装置的情况下 ，一夜之内能够偷窃到的最高金额。


// 如果不偷窃第i间房屋，则f(i) = f(i-1)
// 如果偷窃第i间房屋，则不能偷窃第i-1间房屋，只能从第i-2间房屋开始计算，f(i) = f(i-2) + nums[i]
// f(i) = max(f(i-1), f(i-2) + nums[i])
pub fn rob(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    
    if n == 1 {
        return nums[0];
    }

    let mut f0 = nums[0];
    let mut f1 = f0.max(nums[1]);

    for i in 2..n {
        let f = f1.max(f0 + nums[i]);
        f0 = f1;
        f1 = f;
    }
    f1

}

fn main() {
    let nums = vec![2, 7, 9, 3, 1];
    println!("{}", rob(nums));
}
