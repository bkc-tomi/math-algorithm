/**
 * 与えられた正の整数nが素数かどうかを判定するプログラム
 * 制約 (2 <= n <= 18446744073709551615)
 */
pub fn isprime(n: u64) -> bool {
    let mut i:u64 = 2;

    if n == 2 || n == 3 {
        return true;
    }

    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    return true;
}

pub fn isprime_debug(n: u64) -> bool {
    let mut i:u64 = 2;

    if n == 2 || n == 3 {
        return true;
    }
    
    while i * i <= n {
        if n % i == 0 {
            println!("反復回数: {}", i - 2);
            return false;
        }
        i += 1;
    }
    println!("反復回数: {}", i - 2);
    return true;
}