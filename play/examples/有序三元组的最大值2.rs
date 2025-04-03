
pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
    if nums.len() < 3 {
        return 0;
    }
    let mut numsi: i64 = nums[0] as i64;
    let mut dist: i64 = 0;
    let mut max: i64 = 0;
    for k in 0..nums.len() {
        max = max.max(if dist < 0 { 0 } else { dist * nums[k] as i64 });
        numsi = numsi.max(nums[k] as i64);
        dist = dist.max(numsi - nums[k] as i64);
    }
    max as i64
}
fn main() {
    let a = vec![12, 6, 1, 2, 7];
    println!("{}", maximum_triplet_value(a))
}
