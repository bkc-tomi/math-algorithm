/**
 * 長さn, 要素aiの配列cardsが与えられて、その中から和が100000となる2枚のカードの選びかたは何通りあるか求めるプログラム
 * 制約: (2 <= n <= 200000), (1 <= ai <= 99999)
 */
pub fn comb(cards: &[u64]) -> u64 {
    let mut ans: u64 = 0;
    let l: usize = cards.len();

    for i in 0..l {
        for j in i+1..l {
            if cards[i] + cards[j] == 100000 {
                ans += 1;
            }
        }
    }

    return ans;
}

pub fn comb_imp(cards: &[u64]) -> u64 {
    let mut ans: u64 = 0;
    let mut cnt: [u64; 100000] = [0; 100000];

    for i in 0..cards.len() {
        cnt[cards[i] as usize] += 1;
    }

    for i in 1..50000 {
        ans += cnt[i] * cnt[100000 - i];
    }

    ans += cnt[50000] * (cnt[50000] - 1) / 2;

    return ans;
}