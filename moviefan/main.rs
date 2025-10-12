/// Solution to Movie Fan Problem (https://www.spoj.com/problems/MOVIFAN/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_u64_vec() -> Vec<u64> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn compute_movie_watch_ways_cnt(start_times: &Vec<u64>, show_length: u64) -> usize {

    let modulo = 1000000007;

    let mut biggest_non_overlapping = vec![-1; start_times.len()];
    let mut current_pos = -1;
    for idx in 0..start_times.len() {
        
        while current_pos < idx as i64 && 
              start_times[(current_pos + 1) as usize] + show_length <= start_times[idx] {
            current_pos += 1;
        }
        
        biggest_non_overlapping[idx] = current_pos;
    }

    let mut dp = vec![1; start_times.len()];

    for idx in 1..start_times.len() {
        dp[idx] += dp[idx - 1];
        if biggest_non_overlapping[idx] != -1 {
            dp[idx] += dp[biggest_non_overlapping[idx] as usize];
        }
        dp[idx] %= modulo;
    }

    return *dp.last().unwrap();
}


fn main() {
    let test_case_cnt = read_line_to_usize();
    for _ in 0..test_case_cnt {
        let n_and_l = read_line_to_u64_vec();
        let show_length = n_and_l[1];
        let start_times = read_line_to_u64_vec();
        
        let result = compute_movie_watch_ways_cnt(&start_times, show_length);
        println!("{result}", result=result);
    }
}