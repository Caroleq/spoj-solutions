/// Solution to Sum of Primes (https://www.spoj.com/problems/MPRIME1/)
/// Note: this problem could be solved faster by precomputing all (or at least part of) possible solutions (maximum number on input is 11000)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn generate_primes_up_to(number: usize) -> Vec<u32> {

    let mut primes = Vec::new();

    let mut prime_flags = vec![0; number + 1];
    let number_sqrt = (number as f64).sqrt() as usize + 1;
    for value in 2..number_sqrt {
        if prime_flags[value] == 0 {
            primes.push(value as u32);
        }
        else {
            continue;
        }

        let mut value_multiples = value * value;
        while value_multiples < number + 1 {
            prime_flags[value_multiples] = 1;
            value_multiples += value;
        }
    }

    for value in number_sqrt..number + 1 {
        if prime_flags[value] == 0 {
            primes.push(value as u32);
        }
    }
    return primes;
}

fn compute_sums_number(maximum_number: usize) -> Vec<u64> {

    let primes = generate_primes_up_to(maximum_number);

    let mut dp: Vec<Vec<u64>> = vec![vec![0; maximum_number + 1]; primes.len()];

    for prime_idx in 0..dp.len() {
        for target_number in 0..dp[0].len() {

            if prime_idx > 0 {
                dp[prime_idx][target_number] += dp[prime_idx - 1][target_number];
            }

            if primes[prime_idx] as usize == target_number {
                dp[prime_idx][target_number] += 1;
                continue;
            }
            
            let mut primes_sum = primes[prime_idx];
            let mut prev_prime_idx = prime_idx;
            
            while prev_prime_idx > 0 && (primes_sum as usize) < target_number {
                primes_sum += primes[prev_prime_idx - 1];
                prev_prime_idx -= 1;
            }
            if target_number == primes_sum as usize {
                dp[prime_idx][target_number] += 1;
            }
        }
    }

    let mut result = vec![0; maximum_number + 1];
    for number in 0..maximum_number + 1 {
        result[number as usize] = dp[primes.len() - 1][number];
    }

    return result;
} 

fn main() {

    let mut queries = Vec::new();

    loop {
        let test_case = read_line_to_usize();
        if test_case == 0 {
            break;
        }
        queries.push(test_case);
    }

    let maximum = *queries.iter().max().unwrap();
    let counts = compute_sums_number(maximum);

    for query in queries {
        println!("{case_result}", case_result=counts[query]);
    }
}