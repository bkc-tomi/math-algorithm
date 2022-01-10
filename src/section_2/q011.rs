/**
 * 正の整数nが与えられて、n以下の素数とその個数を返すプログラム
 * 制約: (1 <= n <= 65535)
 */
pub fn show_prime_number(n: u16) -> (Vec<u16>, u16) {
    let mut prime_arr:Vec<u16> = vec![2];
    let mut l: u16 = 1;
    for i in 3..n + 1 {
        let mut flug: bool = true;
        for j in prime_arr.iter() {
            if i % j == 0 {
                flug = false;
                break;
            }
        }
        if flug {
            prime_arr.push(i);
            l += 1;
        }
    }

    return (prime_arr, l);
}