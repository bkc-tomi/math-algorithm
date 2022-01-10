/**
 * 与えられた正の整数nを素因数分解し、配列で返すプログラム
 * 制約 (2 <= n <= 18446744073709551615)
 * ＜例＞
 * 9991       -> [97, 103]
 * 999975     -> [3, 5, 5, 67, 199]
 * 5767       -> [73, 79]
 * 2257       -> [37, 61]
 * 1282881263 -> [13297, 96479]
 * 2021       -> [43, 47]
 */
pub fn prime_factorization(n: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = vec![];
    let mut i:u64 = 2;
    let mut mut_n: u64 = n;

    while i * i <= mut_n {
        if mut_n % i == 0 {
            // 割り切れたら追加
            mut_n = mut_n / i;
            primes.push(i);
        } else {
            // そうでない場合はカウントアップ
            i += 1;
        }
    }
    primes.push(mut_n);
    return primes;
}

pub fn prime_factorization_debug(n: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = vec![];
    let mut i:u64 = 2;
    let mut count: u64 = 1;
    let mut mut_n: u64 = n;

    while i * i <= mut_n {
        if mut_n % i == 0 {
            // 割り切れたら追加
            mut_n = mut_n / i;
            primes.push(i);
        } else {
            // そうでない場合はカウントアップ
            i += 1;
        }
        println!("{:10}回目  i: {:10}, mut_n: {:20}",count, i, mut_n);
        count += 1;
    }
    primes.push(mut_n);
    return primes;
}