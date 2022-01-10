/**
 * 整数n, x, yが与えられn以下の正の整数の中でxの倍数またはyの倍数であるものの個数を出力するプログラム
 * example: n = 15, x = 3, y = 5 -> 7(3, 5, 6, 9, 10, 12, 15)
 * 制約: (1 <= n <= 10^6), (1 <= x < y <= 10^6)
 */
pub fn count_multiple(n: u16, x: u16, y: u16) -> u8 {
    let mut count: u8 = 0;
    for i in 1..n+1 {
        if i % x == 0 || i % y == 0 {
            count += 1;
        }
    }

    return count;
}