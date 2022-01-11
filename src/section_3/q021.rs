/**
 * nCrを求めるプログラム
 * 制約: (1 <= r <= n <= 20)
 */
pub fn comb(r: u8, n: u8) -> u32 {
    let mut temp_n: u32 = n as u32;
    let mut temp: u32 = if r < n - r { r as u32 } else { (n - r) as u32 };

    let mut top:    u32 = 1;
    let mut bottom: u32 = 1;
    
    while temp > 0 {
        top *= temp_n;
        bottom *= temp;

        temp_n -= 1;
        temp -= 1;
    }
    
    return top / bottom;
}