/**
 * 与えられた正の整数a, bの最大公約数を求めるプログラム
 * 制約: (2 <= n <= 18446744073709551615)
 */
pub fn gcd(a: u64, b: u64) -> u64 {
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
        return mut_a;
    } else {
        return mut_b;
    }
}

pub fn gcd_debug(a: u64, b: u64) -> u64 {
    let mut mut_a: u64 = a;
    let mut mut_b: u64 = b;
    let mut temp: u64 = 0;

    while mut_a >= 1 && mut_b >= 1 {
        if mut_a < mut_b {
            temp = mut_b;
            mut_b = mut_b % mut_a;
            println!("{:10} / {:10} = {:10} 余り {:10}", temp, mut_a, temp / mut_a, mut_b);
        } else {
            temp = mut_a;
            mut_a = mut_a % mut_b;
            println!("{:10} / {:10} = {:10} 余り {:10}", temp, mut_b, temp / mut_b, mut_a);
        }
    }

    if mut_a > mut_b {
        return mut_a;
    } else {
        return mut_b;
    }
}