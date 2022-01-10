use std::time::{Instant};

mod section_2;
mod my_utils;

fn main() {
    let start = Instant::now();
    
    section_2::q011::show_prime_number(65000);
    let end = start.elapsed();

    println!("{}.{:06}秒経過しました。", end.as_secs(), end.subsec_nanos() / 1_000_000);
}
