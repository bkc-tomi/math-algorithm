/**
 * n個の商品の値段が入った配列nsから、２つの商品を買うとき、合計の値段が500円に
 * なるような組合せは何通りあるか求めるプログラム
 * 制約: (2 <= n <= 200000), 値段: 100, 200, 300, 400のいずれか。
 */
pub fn comb(ns: &[u16]) -> u32 {
    let mut count_100: u32 = 0;
    let mut count_200: u32 = 0;
    let mut count_300: u32 = 0;
    let mut count_400: u32 = 0;

    for i in ns {
        match i {
            100 => count_100 += 1,
            200 => count_200 += 1,
            300 => count_300 += 1,
            400 => count_400 += 1,
            _ => continue,
        }
    }

    return count_100 * count_400 + count_200 * count_300;
}