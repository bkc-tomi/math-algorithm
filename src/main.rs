use std::time::{Instant};

mod section_3;
mod my_utils;

fn main() {
    let start = Instant::now();
    
    let n: i32 = section_3::q0262::monteCarlo();

    println!("{}", n);
    
    let end = start.elapsed();

    println!("{}.{:06}秒経過しました。", end.as_secs(), end.subsec_nanos() / 1_000_000);
}
