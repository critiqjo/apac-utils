use std::io::{self, BufRead};

fn mod_exp(n: u64, mut p: u64, k: u64) -> u64 { // n^p mod k
    if p == 0 { return 1; }
    let mut powers: Vec<u64> = vec![];
    let mut t = n % k;
    while p > 0 {
        if p % 2 == 1 {
            powers.push(t);
        }
        t = (t * t) % k;
        p /= 2;
    }
    let mut res = 1;
    for t in powers {
        res *= t;
        res %= k;
    }
    res
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines().map(|t| t.unwrap());
    let t: u8 = input.next().unwrap().parse().unwrap();
    for c in 0..t {
        let case: Vec<u64> = input.next().unwrap()
                                  .split_whitespace()
                                  .map(|n| n.parse().unwrap())
                                  .collect();
        let (a, b, n, k) = (case[0], case[1], case[2], case[3]);
        let mut total = 0;
        for i in 1..n+1 {
            for j in i+1..n+1 {
                if (mod_exp(i, a, k) + mod_exp(j, b, k)) % k == 0 {
                    total += 1;
                }
                if (mod_exp(j, a, k) + mod_exp(i, b, k)) % k == 0 {
                    total += 1;
                }
            }
        }
        println!("Case #{}: {}", c+1, total)
    }
}
