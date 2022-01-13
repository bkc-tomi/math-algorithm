/**
 * ２つのサイコロがある。サイコロの出目を配列dice_1, dice_2で与える。
 * サイコロを同時にふり、出目の合計だけ賞金が貰える。貰える賞金の期待値を計算するプログラム
 * 制約
 * dice_1: B1, B2, ..., Bnが等確率で出る。
 * dice_2: R1, R2, ..., Rnが等確率で出る。
 * (2 <= N <= 100000), (0 <= Bi, Ri <= 100)
 */
pub fn dice_expected_value(dice_1: &[u64], dice_2: &[u64]) -> (f64, String) {
    let mut expected_value: f64 = 0.0;
    let mut err_msg: String = String::new();

    if dice_1.len() != dice_2.len() {
        err_msg = String::from("与えられた引数の長さが異なります。");
        
        return (expected_value, err_msg);
    } else {
        let p_val:f64 = 1.0 / dice_1.len() as f64;

        let mut dice_1_ev:f64 = 0.0;
        let mut dice_2_ev:f64 = 0.0;

        for i in 0..dice_1.len() {
            dice_1_ev += dice_1[i] as f64 * p_val;
            dice_2_ev += dice_2[i] as f64 * p_val;
        }

        expected_value = dice_1_ev + dice_2_ev;

        return (expected_value, err_msg);
    }
}