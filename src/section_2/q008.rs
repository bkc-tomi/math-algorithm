/**
 * 赤・青のカードが各1枚ずつあり、それぞれのカードに１以上N以下の整数を１つ書き込む。カードに書かれた整数の合計がS以下となるような書き方がいくつあるか出力するプログラム。
 * example: N=3, S=4 -> 6
 * 制約: (1 <= N <= 1000), (1 <= S <= 2000)
 */
pub fn count_convination(n: u16, s: u16) -> u32 {
    let mut count: u32 = 0;
    for i in 1..n + 1 {
        for j in 1..n + 1 {
            if i + j <= s {
                count += 1;
            }
        }
    }
    return count;
}

pub fn count_convination_debug(n: u16, s: u16) -> u32 {
    let mut count: u32 = 0;
    let mut count_debug: u32 = 0;
    for i in 1..n + 1 {
        for j in 1..n + 1 {
            if i + j <= s {
                count += 1;
            }
            count_debug += 1;
            println!("{}回目: {}個", count_debug, count);
        }
    }
    return count;
}