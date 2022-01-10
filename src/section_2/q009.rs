/**
 * 配列anで与えられた数字のリストから合計がsとなるような組み合わせは存在するかどうか判定するプログラム
 * 制約: (1 <= n <= 60, 1 <= ai <= 10000, 1 <= s <= 10000)
 */
pub fn choose_card(s: u16, an: &[u16]) -> bool {
    let n:usize = an.len();
    let mut partsum: u16 = 0;

    for i in 0..1 << n {
        partsum = 0;
        for j in 0..n {
            if i & (1 << j) != 0 {
                partsum += an[j];
            }
        }
        if partsum == s {
            return true;
        }
    }
    return false;
}

pub fn choose_card_debug(s: u16, an: &[u16]) -> bool {
    let mut b: bool = false;
    let n:usize = an.len();
    let mut partsum: u16 = 0;
    for i in 0..1 << n {
        partsum = 0;
        println!("partsum init.");
        for j in 0..n {
            if i & (1 << j) != 0 {
                println!("i: {}, j: {}, i & (1 << j): {}, add partsum.", i, j, i & (1 << j));
                partsum += an[j];
            } else {
                println!("i: {}, j: {}, i & (1 << j): {}", i, j, i & (1 << j));
            }
        }
        println!("{}", partsum);
        if partsum == s {
            b = true;
        }
    }
    return b;
}