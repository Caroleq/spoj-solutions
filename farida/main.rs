/// Solution to Princess Farida (https://www.spoj.com/problems/FARIDA/)

fn read_line_to_usize() -> usize {
   let mut string = String::new();
   let _ = std::io::stdin().read_line(&mut string);
   return string.trim().parse().expect("Input not an integer");
}

fn read_empty_line() {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
}

fn read_line_to_u64_vec() -> Vec<u64> {
   let mut string = String::new();
   let _ = std::io::stdin().read_line(&mut string);
   return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn maximum_coins(coin_vals: &Vec<u64>) -> u64 {
    if coin_vals.len() == 0 {
        return 0;
    }
    else if coin_vals.len() == 1 {
        return coin_vals[0];
    }
    else if coin_vals.len() == 2 {
        return std::cmp::max(coin_vals[0], coin_vals[1]);
    }
    
    let n = coin_vals.len();
    let mut dp = vec![0; n];
    dp[0] = coin_vals[0];
    dp[1] = std::cmp::max(coin_vals[0], coin_vals[1]);

    for idx in 2..n {
        dp[idx] = std::cmp::max(dp[idx - 1], coin_vals[idx] + dp[idx - 2]);
    }

    return dp[n - 1];
}

fn main() {

    let test_case_cnt = read_line_to_usize();
    for case in 0..test_case_cnt {
        let coins_len = read_line_to_usize();
        if coins_len == 0 {
            println!("Case {case}: 0", case=case + 1);
            read_empty_line();
            continue;
        }
        let coins = read_line_to_u64_vec();

        let result = maximum_coins(&coins);
        println!("Case {case}: {result}", case=case + 1, result=result);
    }

}