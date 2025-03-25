// 给你一个整数 n 和一个在范围 [0, n - 1] 以内的整数 p ，它们表示一个长度为 n 且下标从 0 开始的数组 arr ，数组中除了下标为 p 处是 1 以外，其他所有数都是 0 。

// 同时给你一个整数数组 banned ，它包含数组中的一些位置。banned 中第 i 个位置表示 arr[banned[i]] = 0 ，题目保证 banned[i] != p 。

// 你可以对 arr 进行 若干次 操作。一次操作中，你选择大小为 k 的一个 子数组 ，并将它 翻转 。在任何一次翻转操作后，你都需要确保 arr 中唯一的 1 不会到达任何 banned 中的位置。换句话说，arr[banned[i]] 始终 保持 0 。

// 请你返回一个数组 ans ，对于 [0, n - 1] 之间的任意下标 i ，ans[i] 是将 1 放到位置 i 处的 最少 翻转操作次数，如果无法放到位置 i 处，此数为 -1 。

// 子数组 指的是一个数组里一段连续 非空 的元素序列。
// 对于所有的 i ，ans[i] 相互之间独立计算。
// 将一个数组中的元素 翻转 指的是将数组中的值变成 相反顺序 。

use std::collections::{BTreeSet, HashSet, VecDeque};

pub fn min_reverse_operations(n: i32, p: i32, banned: Vec<i32>, k: i32) -> Vec<i32> {
    let n = n as usize;
    let p = p as usize;
    let k = k as usize;
    let mut ans = vec![-1; n as usize];
    let mut indices = vec![BTreeSet::new(); 2];
    let banned = banned.into_iter().collect::<HashSet<_>>();
    for i in 0..n {
        if i != p && !banned.contains(&(i as i32)) {
            indices[i as usize % 2].insert(i);
        }
    }

    let mut q = VecDeque::<i32>::new();
    q.push_back(p as i32);
    ans[p] = 0;

    while !q.is_empty() {
        let i = q.pop_front().unwrap();
        let i = i as usize;
        let mn = (i as i32 - k as i32 + 1).max(k as i32 - i as i32 - 1) as usize;
        let mx = (i as i32 + k as i32 - 1).min(n as i32 * 2 - k as i32 - i as i32 - 1) as usize;

        // let set = indices.get_mut(mx % 2).unwrap();
        let set = &mut indices[mx % 2];

        let mut need_remove = vec![];
        for &j in set.range(mn..=mx) {
            ans[j as usize] = ans[i as usize] + 1;
            q.push_back(j as i32);
            need_remove.push(j);
            // set.remove(&j);
        }
        for ele in need_remove {
            set.remove(&ele);
        }
    }
    ans
}

fn main() {
    let n = 4;
    let p = 3;
    let banned = vec![];
    let k = 2;
    println!("{:?}", min_reverse_operations(n, p, banned, k));
}
