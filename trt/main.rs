///  Solution to Treats for the Cows (TRT) Problem (https://www.spoj.com/problems/TRT//)

fn read_line_to_u32() -> u32 {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_lines_to_u32_vec(read_cnt: usize) -> Vec<u32> {
    let mut vec = Vec::new();

    for _ in 0..read_cnt {
        vec.push(read_line_to_u32());
    }

    return vec;
}

fn compute_max_revenue(treats: &Vec<u32>, dp: &mut Vec<Vec<u32>>, partial_sums: &Vec<u32>, begin: usize, end: usize) -> u32 {

    if dp[begin][end] > 0 {
        return dp[begin][end];
    }

    if begin == end {
        dp[begin][end] = treats[begin];
        return dp[begin][end];
    }

    let mut subsum = partial_sums[end];
    if begin > 0 {
        subsum -= partial_sums[begin - 1] ;
    }

    let left_revenue =  compute_max_revenue(treats, dp, partial_sums, begin + 1, end);
    let right_revenue = compute_max_revenue(treats, dp, partial_sums, begin, end - 1);
    dp[begin][end] = subsum + std::cmp::max(left_revenue, right_revenue);

    return dp[begin][end];
}

fn main() {
    let problem_size = read_line_to_u32() as usize;
    let treats = read_lines_to_u32_vec(problem_size);

    let mut partial_sums = vec![0; problem_size];
    partial_sums[0] = treats[0];
    for i in 1..problem_size {
        partial_sums[i] = partial_sums[i - 1] + treats[i];
    }

    let mut dp = vec![vec![0; problem_size]; problem_size];
    let max_revenue = compute_max_revenue(&treats, &mut dp, &partial_sums, 0, problem_size - 1);

    println!("{max_revenue}", max_revenue=max_revenue);
}