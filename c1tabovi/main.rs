/// Solution to Tablovi (https://www.spoj.com/problems/C1TABOVI/)

fn read_line_to_i32_vec() -> Vec<i32> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn compute_minimum_shifts_cnt(starting_pos: &Vec<i32>, destination_pos: &Vec<i32>) -> i32 {

    let problem_size = starting_pos.len();
    let mut dp = vec![0; problem_size];

    dp[0] = (starting_pos[0] - destination_pos[0]).abs();
    for idx in 1..problem_size {

        let dist_directed = destination_pos[idx] - starting_pos[idx];
        let dist_prev_directed = destination_pos[idx - 1] - starting_pos[idx - 1];

        if dist_directed == 0 {
            dp[idx] = dp[idx - 1];
            continue;
        }

        let same_direction = dist_prev_directed < 0 && dist_directed < 0 ||
                             dist_prev_directed > 0 && dist_directed > 0;

        if same_direction {
            if dist_prev_directed.abs() > dist_directed.abs() {
                dp[idx] = dp[idx - 1];
            }
            else {
                dp[idx] = dp[idx - 1] + dist_directed.abs() - dist_prev_directed.abs() 
            }
        }
        else {
            dp[idx] = dp[idx - 1] + dist_directed.abs();
        }
    }

    return *dp.last().unwrap();
}


fn main() {
    let _ = read_line_to_i32_vec();

    let starting_pos = read_line_to_i32_vec();
    let destination_pos = read_line_to_i32_vec();

    let result = compute_minimum_shifts_cnt(&starting_pos, &destination_pos);
    println!("{result}", result=result);
}