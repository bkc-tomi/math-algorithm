/**
 * 正の整数nの約数を配列で返すプログラム
 * 制約 (1 <= n <= 18446744073709551615)
 */
pub fn divsors(n: u64) -> Vec<u64> {
    let mut divs: Vec<u64> = vec![];
    let mut i: u64 = 1;

    while i * i <= n {
        if n % i == 0 {
            divs.push(i);
            if i != n / i {
                divs.push(n / i);
            }
        }
        i += 1;
    }
    divs.sort();
    return divs;
}

pub fn divsors_debug(n: u64) -> Vec<u64> {
    let mut divs: Vec<u64> = vec![];
    let mut i: u64 = 1;

    while i * i <= n {
        if n % i == 0 {
            divs.push(i);
            if i != n / i {
                divs.push(n / i);
            }
        }
        println!("{}回目: divs: {:?}", i, divs);
        i += 1;
    }
    divs.sort();
    return divs;
}