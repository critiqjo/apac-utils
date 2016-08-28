use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines().map(|t| t.unwrap());
    let t: u8 = input.next().unwrap().parse().unwrap();
    for c in 0..t {
        let case: Vec<u64> = input.next().unwrap()
                                  .split_whitespace()
                                  .map(|n| n.parse().unwrap())
                                  .collect();
        let min_lr = std::cmp::min(case[0], case[1]);
        println!("Case #{}: {}", c+1, min_lr*(min_lr + 1)/2);
    }
}
