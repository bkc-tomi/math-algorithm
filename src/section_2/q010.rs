/**
 * 整数nが与えられて, n!を出力するプログラム
 * 制約: (1 <= n <= 20)
 */
pub fn prod(n: u64) -> u64 {
    let mut p: u64 = 1;

    for i in 1..n + 1 {
        p *= i;
    }

    return p;
}

pub fn prod_debug(n: u64) -> u64 {
    let mut p: u64 = 1;

    for i in 1..n + 1 {
        p *= i;
        println!("{:2}回目: n!={:20}", i, p);
    }

    return p;
}