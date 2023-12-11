/// Solution to Piggy-Bank Problem (https://www.spoj.com/problems/PIGBANK/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_lines_to_u32_vecs(strings_cnt: usize) -> Vec<Vec<u32>> {
    let mut strings = Vec::new();
    for _ in 0..strings_cnt {
        let mut string = String::new();
        let _ = std::io::stdin().read_line(&mut string);
        strings.push(string.trim().split(' ').map(|s| s.parse().unwrap()).collect());
    }
    return strings;
}

fn read_line_to_usize_vec() -> Vec<usize> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn compute_min_value(coins: &Vec<Vec<u32>>, coins_weight: usize) -> u32 {

    let mut dp = vec![std::u32::MAX; coins_weight + 1];

    for weight in 1..coins_weight + 1 {
        for coin in coins {
            if (weight as u32) >= coin[1] && dp[weight - coin[1] as usize] != std::u32::MAX {
                dp[weight] = std::cmp::min(dp[weight], dp[weight - coin[1] as usize] + coin[0]);
            }
            
            if weight as u32 == coin[1] {
                dp[weight] = std::cmp::min(dp[weight], coin[0]);
            }
        }
    }

    return dp[coins_weight];
}

fn main() {
    let cases_count = read_line_to_usize();
    for _ in 0..cases_count {
        let e_and_f = read_line_to_usize_vec();
        let coins_cnt = read_line_to_usize();
        let coins_n_weights = read_lines_to_u32_vecs(coins_cnt);

        let empty_pig_weight = e_and_f[0];
        let filled_pig_weight = e_and_f[1];
        if empty_pig_weight == filled_pig_weight {
            println!("The minimum amount of money in the piggy-bank is 0.");
            continue;
        }
        let min_value = compute_min_value(&coins_n_weights, filled_pig_weight - empty_pig_weight);
        if min_value == std::u32::MAX {
            println!("This is impossible.");
        }
        else {
            println!("The minimum amount of money in the piggy-bank is {min_value}.", min_value=min_value);
        }
    }
}