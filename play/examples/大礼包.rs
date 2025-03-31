use std::collections::HashMap;
// dfs
pub fn shopping_offers(price: Vec<i32>,mut special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
    let mut need_remove = Vec::new();
    for (i,s) in special.iter_mut().enumerate() {
        let mut total = 0;
        for (j, n) in s[0..s.len() - 1].iter().enumerate() {
            total += n * price[j];
            if s[j] > needs[j] {
                need_remove.push(i);
            }
        }
        if total < s[s.len()-1] {
            need_remove.push(i);
        }
        s.push(total-s[s.len()-1]);
        
    }
    let special: Vec<Vec<i32>> = special.into_iter().enumerate().filter(|(i,_)| !need_remove.contains(&i)).map(|(_,s)| s).collect();
    println!("{:?}", special);
    let mut memo = HashMap::new();

    fn dfs(price: &Vec<i32>, special: &Vec<Vec<i32>>, cur_needs: Vec<i32>, memo: &mut HashMap<Vec<i32>, i32>) -> i32 {
        if let Some(res) = memo.get(&cur_needs) {
            return *res;
        }
        let n = cur_needs.len();
        let mut min_price = 0;
        for i in 0..n {
            min_price += price[i] * cur_needs[i];
        }

        for cur_special in special.iter() {
            let special_price = cur_special[cur_special.len()-2];
            let mut next_needs = Vec::new();
            for i in 0..n {
                if cur_needs[i] < cur_special[i] {
                    break;
                }
                next_needs.push(cur_needs[i] - cur_special[i]);
            }
            if next_needs.len() == n {
                min_price = min_price.min(dfs(price, special, next_needs, memo) + special_price);
            }
        }
        memo.insert(cur_needs, min_price);
        min_price
    }

    dfs(&price, &special, needs, &mut memo)
}


fn main() {
    let price = vec![2,5];
    let special = vec![vec![3,0,5],vec![1,2,10]];
    let needs = vec![3,2];
    println!("{}", shopping_offers(price, special, needs));
}
