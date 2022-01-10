/**
 * 与えられた２つ整数a, bの最小公倍数を求めるプログラム
 * 制約: (2 <= n <= 18446744073709551615)
 */
pub fn lcm(a: u64, b: u64) -> u64 {
    // lcm(a, b) = ab / gcd(a, b)の性質を使用する
    let mut mut_a: u64 = a;
    let mut mut_b: u64 = b;

    while mut_a >= 1 && mut_b >= 1 {
        if mut_a < mut_b {
            mut_b = mut_b % mut_a;
        } else {
            mut_a = mut_a % mut_b;
        }
    }

    if mut_a > mut_b {
        return a * b / mut_a;
    } else {
        return a * b / mut_b;
    }
}

/**
 * 配列numsで与えられたN個の整数aiの最小公倍数を求めるプログラム
 * 制約: (2 <= ai <= 18446744073709551615)
 */
pub fn multi_lcm(nums: &[u64]) -> u64 {
    let mut lcm_num: u64 = nums[0];

    for i in 1..nums.len() {
        lcm_num = lcm(lcm_num, nums[i]);
    }

    return lcm_num;
}

