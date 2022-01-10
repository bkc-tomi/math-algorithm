use std::time::{Instant};

mod section_3;
mod my_utils;

fn main() {
    let start = Instant::now();
    let nums: [u64; 3] = [120, 156, 180];
    let num: u64 = section_3::q017::multi_lcm(&nums);

    println!("lcm: {}", num);
    
    let end = start.elapsed();

    println!("{}.{:06}秒経過しました。", end.as_secs(), end.subsec_nanos() / 1_000_000);
}
