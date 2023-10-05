/// Solution to Ada and Game (ADAGAME) problem: https://www.spoj.com/problems/ADAGAME/

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

fn increment_ith_digit(number: usize, digit_pos: usize) -> usize {
    let base: usize = 10;
    let exp_value = base.pow((digit_pos) as u32);
    let digit = (number / exp_value) % 10;

    if digit == 9 {
        return number - digit * exp_value;
    }

    return number + exp_value;
}

fn can_ada_win_ada_turn(initial_number: usize, current_number: usize, turns_left: usize, dp: &mut Vec<Vec<i8>>) -> bool {

    if turns_left == 0 {
        return initial_number < current_number;
    }

    if dp[turns_left - 1][current_number] != -1 {
        return dp[turns_left - 1][current_number] == 1;
    }
    
    for digit in 0..4 {
        if can_ada_win_vinit_turn(initial_number, increment_ith_digit(current_number, digit), turns_left - 1, dp) {
            dp[turns_left - 1][current_number] = 1;
            return true;
        }

    }

    dp[turns_left - 1][current_number] = 0;
    return false;

}

fn can_ada_win_vinit_turn(initial_number: usize, current_number: usize, turns_left: usize, dp: &mut Vec<Vec<i8>>) -> bool {

    if turns_left == 0 {
        return initial_number < current_number;
    }

    if dp[turns_left - 1][current_number] != -1 {
        return dp[turns_left - 1][current_number] == 1;
    }

    for digit in 0..4 {
        if !can_ada_win_ada_turn(initial_number, increment_ith_digit(current_number, digit), turns_left - 1, dp) {
            dp[turns_left - 1][current_number] = 0;
            return false;
        }
    }

    dp[turns_left - 1][current_number] = 1;
    return true;

}

fn main() {
    let case_count = read_line_to_usize();
    
    for _ in 0..case_count {
        let case = read_line_to_usize_vec();
        let initial_number = case[0];
        let turns_number = case[1];

        let mut dp: Vec<Vec<i8>> = vec![vec![-1; 10000]; turns_number ];

        if can_ada_win_ada_turn(initial_number, initial_number, turns_number, &mut dp) {
            println!("Ada");
        }
        else {
            println!("Vinit");
        }
    }
}