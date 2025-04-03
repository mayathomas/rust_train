// f[i] = f[i-1] == 0 ? 0 : f[i]
pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let n = nums.len();
    let mut f = vec![-1; n - k as usize + 1];

    let sorted = |a, b| a < b;

    let mut start = 0;
    let mut end = k as usize - 1;
    let mut is_sorted =
        nums[start..=end].is_sorted_by(sorted) && nums[end] - nums[start] == (end - start) as i32;
    // println!("{:?}", is_sorted);
    while end < n as usize {
        if is_sorted {
            f[start] = nums[end] as i32;
        }
        start += 1;
        end += 1;
        if end < n as usize {
            is_sorted = nums[start..=end].is_sorted_by(sorted)
                && nums[end] - nums[start] == (end - start) as i32;
        }
    }
    f
}
fn main() {
    // let a = vec![3, 2, 3, 2, 3, 2];
    // let a = vec![1];
    let a = vec![1, 3, 4];
    let b = results_array(a, 2);
    println!("{:?}", b)
}
