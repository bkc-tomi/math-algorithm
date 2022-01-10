/**
 * N個の整数a1, ..., aNが配列anで与えられる。それを全て足して100で割った余りを計算するプログラム
 */
pub fn sum_remainder(an: &[u32]) -> u32 {
    let mut sum: u32 = 0;
    for a in an {
        sum += a;
    }
    let r: u32 = sum % 100;

    return r;
}