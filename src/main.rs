use std::time::{Instant};

mod section_3;
mod my_utils;

fn main() {
    let start = Instant::now();

    println!("{:?}", section_3::q014::prime_factorization(11));
    
    let end = start.elapsed();

    println!("{}.{:06}秒経過しました。", end.as_secs(), end.subsec_nanos() / 1_000_000);
}
