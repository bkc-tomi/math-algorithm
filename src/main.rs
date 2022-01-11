use std::time::{Instant};

mod section_3;
mod my_utils;

fn main() {
    let start = Instant::now();
    let cards: [u64; 6] = [40000, 50000, 20000, 80000, 50000, 30000];
    let num = section_3::q022::comb(&cards);

    println!("lcm: {}", num);
    
    let end = start.elapsed();

    println!("{}.{:06}秒経過しました。", end.as_secs(), end.subsec_nanos() / 1_000_000);
}
