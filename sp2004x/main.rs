// Solution to Bankomat Problem (https://www.spoj.com/problems/SP2004X/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_i64_vec() -> Vec<i64> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn if_can_withdraw_requested_amount(tens_cnt: i64, twenties_cnt: i64, fifties_cnt: i64, hundreds_cnt: i64, two_hundreds_cnt: i64, requested_amount: i64) -> bool {

    if requested_amount % 10 != 0 {
        return false;
    }

    let mut requested_amount = requested_amount;

    if requested_amount > two_hundreds_cnt * 200 {
        requested_amount -= two_hundreds_cnt * 200;
    }
    else {
        requested_amount %= 200;
    }

    if requested_amount == 0 {
        return true;
    }

    if requested_amount > hundreds_cnt * 100 {
        requested_amount -= hundreds_cnt * 100
    }
    else {
        requested_amount %= 100;
    }

    if requested_amount == 0 {
        return true;
    }

    let mut left_fifties_cnt = fifties_cnt;
    let mut left_twenties_cnt = twenties_cnt;

    if requested_amount > 100 {
        let number_of_50_in_requested_amount = requested_amount / 50;
        requested_amount -= std::cmp::min(number_of_50_in_requested_amount / 2, fifties_cnt / 2) * 100;
        left_fifties_cnt -= std::cmp::min(number_of_50_in_requested_amount / 2, fifties_cnt / 2) * 2;
        if requested_amount == 0 {
            return true;
        }

        if requested_amount > 100 {
            let number_of_20_in_requested_amount = requested_amount / 20;
            requested_amount -= std::cmp::min(number_of_20_in_requested_amount / 5, twenties_cnt / 5) * 100;
            left_twenties_cnt -= std::cmp::min(number_of_20_in_requested_amount / 5, twenties_cnt / 5) * 5;
            if requested_amount == 0 {
                return true;
            }
        }
    }
    
    for fifties_to_take in 0..std::cmp::min(left_fifties_cnt + 1, 3) {
        for twenties_to_take in 0..std::cmp::min(left_twenties_cnt + 1, 6) {

            let amount_left = requested_amount - fifties_to_take * 50 - twenties_to_take * 20;

            if amount_left >= 0 && amount_left <= tens_cnt * 10 {
                return true;
            }
        }
    }

    return false;
}


fn main() {
    let tests_count = read_line_to_usize();

    for _ in 0..tests_count {
        let test_case = read_line_to_i64_vec();

        let tens_cnt = test_case[0];
        let twenties_cnt = test_case[1];
        let fifties_cnt = test_case[2];
        let hundreds_cnt = test_case[3];
        let two_hundreds_cnt = test_case[4];

        let requested_amount = test_case[5];

        if if_can_withdraw_requested_amount(tens_cnt, twenties_cnt, fifties_cnt, hundreds_cnt, two_hundreds_cnt, requested_amount) {
            println!("TAK");
        }
        else {
            println!("NIE");
        }
    }
}