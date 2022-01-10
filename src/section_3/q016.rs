/**
 * 配列numsに格納されたN個の整数aiの最大公約数を計算するプログラム
 * 制約: (1 <= ai <= 18446744073709551615)
 */
pub fn multi_gcd(nums: &[u64]) -> u64 {
    let mut gcd_num: u64 = nums[0];
    let mut mut_a: u64;
    let mut mut_b: u64;

    for i in 1..nums.len() {
        mut_a = gcd_num;
        mut_b = nums[i];

        while mut_a >= 1 && mut_b >= 1 {
            if mut_a < mut_b {
                mut_b = mut_b % mut_a;
            } else {
                mut_a = mut_a % mut_b;
            }
        }
    
        if mut_a > mut_b {
            gcd_num = mut_a;
        } else {
            gcd_num = mut_b;
        }
    }

    return gcd_num;
}

pub fn multi_gcd_debug(nums: &[u64]) -> u64 {
    let mut gcd_num: u64 = nums[0];
    let mut mut_a: u64;
    let mut mut_b: u64;

    for i in 1..nums.len() {
        mut_a = gcd_num;
        mut_b = nums[i];
        print!("gcd({}, {}) =", mut_a, mut_b);
        while mut_a >= 1 && mut_b >= 1 {
            if mut_a < mut_b {
                mut_b = mut_b % mut_a;
            } else {
                mut_a = mut_a % mut_b;
            }
        }
    
        if mut_a > mut_b {
            gcd_num = mut_a;
        } else {
            gcd_num = mut_b;
        }
        print!("{}\n", gcd_num);
    }

    return gcd_num;
}