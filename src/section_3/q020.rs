/**
 * n枚のカードの配列があり、左からi番目(1 <= i <= n)のカードには整数Aiが書かれています。
 * カードを5枚選ぶ方法のうち、選んだカードに書かれた整数の和がちょうど1000となるものは何通りありますか。
 * 制約: (5 <= n <= 100), (1 <= Ai <= 1000)
 */
pub fn comb(cards: &[u32]) -> u64 {
    let mut ans: u64 = 0;
    let len:usize = cards.len();

    for i in 0..len {
        for j in i+1..len {
            for k in j+1..len {
                for l in k+1..len {
                    for m in l+1..len {
                        if cards[i] + cards[j] + cards[k] + cards[l] + cards[m] == 1000 {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }
    return ans;
}