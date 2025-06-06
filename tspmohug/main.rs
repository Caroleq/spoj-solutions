/// Solution to Hu-Gi-Oh (https://www.spoj.com/problems/TSPMOHUG/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_usize_vec() -> Vec<usize> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn main() {
    let cards_cnt = read_line_to_usize();
    let mut cards = Vec::new();
    for _ in 0..cards_cnt {
        cards.push(read_line_to_usize_vec());
    }

    let mut dp = vec![0; cards_cnt];
    let mut max_previous_power_to_add = 0;

    let mut max_power = 0;

    for idx in 0..cards.len() {
        max_previous_power_to_add = std::cmp::max(
            max_previous_power_to_add, dp[idx]
        );

        dp[idx] = cards[idx][0] + max_previous_power_to_add;
        max_power = std::cmp::max(max_power, dp[idx]);

        let idx_to_use_card = idx + cards[idx][1];
        if idx_to_use_card  + 1 < cards_cnt {
            dp[idx_to_use_card + 1] = std::cmp::max(dp[idx_to_use_card + 1], dp[idx]);
        }
    }

    println!("{result}", result=max_power);
}