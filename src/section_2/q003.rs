/**
 * N個の整数a1, ..., aNが配列として与えられ、それを合計するプログラム
 * 制約: (1 <= N <= 50), (1 <= a1, ..., aN <= 100)
 */
pub fn many_sum(an: &[u16]) -> u16 {
    let mut sum: u16 = 0;
    for a in an {
        sum += a;
    }
    return sum;
}