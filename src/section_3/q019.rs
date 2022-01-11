/**
 * 赤青黄の３種類のカードが合わせてN枚あります。赤: 1, 青: 2, 黄: 3で表し、
 * 配列cardsとして表します。
 * 同じ色のカードを2枚選ぶ方法は何通りあるか求めるプログラム
 * 制約: (2 <= n <= 500000)
 */
pub fn comb(cards: &[u8]) -> u32 {
    let mut red_count   : u32 = 0;
    let mut blue_count  : u32 = 0;
    let mut yellow_count: u32 = 0;

    for c in cards {
        match c {
            1 => red_count += 1,
            2 => blue_count += 1,
            3 => yellow_count += 1,
            _ => continue,
        }
    }

    return red_count * (red_count - 1 ) / 2 
        + blue_count * (blue_count - 1 ) / 2 
        + yellow_count * (yellow_count - 1 ) / 2;
}