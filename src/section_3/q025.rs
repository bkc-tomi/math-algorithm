/**
 * 夏休みはN日間あります。i日目(1 <= i <= N)の勉強時間を以下の手順で決める。
 * - 1日の最初にサイコロをふる
 * - サイコロを振って1, 2: Ai時間勉強する
 * - サイコロを振って3, 4, 5, 6: Bi時間勉強する
 * 合計勉強時間の期待値を求めるプログラム
*/
pub fn expected_value(a: &[u8], b: &[u8]) -> f64 {
    let mut e:f64 = 0.0;

    let ap = 2.0 / 6.0;
    let bp = 4.0 / 6.0;

    for i in 0..a.len() {
        e += a[i] as f64 * ap + b[i] as f64 * bp;
    }

    return e;
}