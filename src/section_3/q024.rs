/**
 * N問からなる選択式のテストがあり、i問目はPi個の選択肢から１つの正解を選ぶ問題であり
 * 配点はQi点である。
 * 全てをランダムに選んだ場合の期待値を求めるプログラム
 * 制約: (1 <= N <= 50), (2 <= Pi <= 9), (1 <= Qi <= 200)
*/
pub fn expected_value(ps: &[u8], qs: &[u8]) -> f64 {
    let mut ev:f64 = 0.0;

    for i in 0..ps.len() {
        ev += qs[i] as f64 / ps[i] as f64;
    }

    return ev;
}