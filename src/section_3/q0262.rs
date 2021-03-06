use rand::Rng;
/**
 * 
 */
pub fn monteCarlo() -> i32 {
    // ========================================================================
    // 初期値
    // ========================================================================
    let mut rng = rand::thread_rng(); // デフォルトの乱数生成器を初期化します
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let N: i32 = 1000000;
    let mut cnt: i32 = 0;
    let mut distance1: f64 = 0.0;
    let mut distance2: f64 = 0.0;
    
    // ========================================================================
    // 円の設定の定義
    // ========================================================================
    // 上の円
    let r1: f64 = 2.0;
    let ox1: f64 = 3.0;
    let oy1: f64 = 7.0;
    
    // 下の円
    let r2: f64 = 3.0;
    let ox2: f64 = 3.0;
    let oy2: f64 = 3.0;
    
    // ========================================================================
    // モンテカルロ実行
    // ========================================================================
    for i in 0..N {
        x = rng.gen::<f64>() * 6.0;
        y = rng.gen::<f64>() * 9.0;

        distance1 = ((x - ox1).powi(2) + (y - oy1).powi(2)).sqrt();
        distance2 = ((x - ox2).powi(2) + (y - oy2).powi(2)).sqrt();

        if distance1 < r1 {
            cnt += 1;
        } else if distance2 < r2 {
            cnt += 1;
        }
    }
    return cnt;
}