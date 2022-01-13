use std::time::{Instant};

mod section_3;
mod my_utils;

fn main() {
    let start = Instant::now();
    
    // let ps:[u8; 5] = [3, 1, 4, 1, 5];
    // let qs:[u8; 5] = [9, 2, 6, 5, 3];
    let n: u64 = 5;
    let ev = section_3::q026::expected_value(n);

    println!("{}", ev);
    
    let end = start.elapsed();

    println!("{}.{:06}秒経過しました。", end.as_secs(), end.subsec_nanos() / 1_000_000);
}
