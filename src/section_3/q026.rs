/**
 * 1ドル払うとN種類のコインのうち１つが等確率で出現する機械がある。
 * 全種類のコインを集めるまでに支払う金額の期待値を計算するプログラム。
 * 制約: (2 <= N <= 1000000)
 * 考え方は以下を参照
 * https://github.com/E869120/math-algorithm-book/blob/main/editorial/chap3-4/chap3-4.pdf
*/
pub fn expected_value(n: u64) -> f64 {
    let mut e:f64 = 0.0;

    for i in 1..n + 1 {
        e += 1.0 * n as f64 / i as f64;
    }
    return e;
}