/// Solution to Fibonacci and Easy GCD (https://www.spoj.com/problems/INS17M/)

const MODULO: u64 = 1000000007;
const PISANO_PERIOD: u64 = 2000000016;

fn compute_gcd_cnt(numbers_cnts: &Vec<u64>) -> Vec<u64> {
    let mut dp = vec![0; numbers_cnts.len()];

    for idx1 in (1..numbers_cnts.len()).rev() {
        let mut gcd_cnt = 0; numbers_cnts[idx1];

        let mut idx2 = idx1;
        while idx2 < numbers_cnts.len() {
            gcd_cnt += numbers_cnts[idx2];
            idx2 += idx1;
        }

        if gcd_cnt == 0 {
            continue;
        }

        gcd_cnt = (gcd_cnt - 1) * gcd_cnt / 2;
        
        let mut idx2 = 2 * idx1;
        while idx2 < numbers_cnts.len() {
            gcd_cnt -= dp[idx2];
            idx2 += idx1;
        }

        dp[idx1] = gcd_cnt;
    }

    return dp;
}

fn fast_fibonacci(exponent: u64) -> (u64, u64, u64, u64) {
    let mut exponent = exponent;
    let mut ans = (1, 0, 0, 1);
    let mut  m = (1, 1, 1, 0);
    
    while exponent > 0 {

        if (exponent % 2) == 1 { 
            ans = (ans.0 * m.0 + ans.1 * m.2, ans.0 * m.1 + ans.1 * m.3, 
               ans.2 * m.0 + ans.3 * m.2, ans.2 * m.1 + ans.3 * m.3);

            ans = (ans.0 % MODULO, ans.1 % MODULO, ans.2 % MODULO, ans.3 % MODULO);
        }

        m = (m.0 * m.0 + m.1 * m.2, m.1 * m.0 + m.1 * m.3, 
                 m.2 * m.0 + m.3 * m.2, m.2 * m.1 + m.3 * m.3);

        m = (m.0 % MODULO, m.1 % MODULO, m.2 % MODULO, m.3 % MODULO);

        exponent /= 2;
    }

    return ans;
}

fn compute_exponent(number: u64, exponent: u64) -> u64 {
    let mut exponent = exponent;

    if exponent == 0 {
        return 1;
    }

    let mut result = number as u128;
    let mut number = number as u128;
    exponent -= 1;

    while exponent > 0 {

        if (exponent % 2) == 1 {
            result *= number;
        }

        result %= PISANO_PERIOD as u128;

        number *= number;
        number %= PISANO_PERIOD as u128;
        exponent /= 2;
    }

    return result as u64;
}

fn fibonacci(number: u64, power: u64) -> u64 {
    let fib_number = compute_exponent(number, power);
    
    let matrix = fast_fibonacci(fib_number);
    return matrix.1;
}

fn compute_number_cnts(numbers: &Vec<u64>) -> Vec<u64> {
    let max_number = *numbers.iter().max().unwrap();

    let mut numbers_cnt = vec![0; max_number as usize + 1];

    for number in numbers {
        numbers_cnt[*number as usize] += 1;
    }
    return numbers_cnt;
}

fn compute_sum(numbers: &Vec<u64>, power: u64) -> u64 { 

    let numbers_cnt = compute_number_cnts(&numbers);
    let gcd_cnts = compute_gcd_cnt(&numbers_cnt);

    let mut result = 0;
    for gcd_value in 1..gcd_cnts.len() {

        if gcd_cnts[gcd_value] == 0 {
            continue;
        }

        result += fibonacci(gcd_value as u64, power) * gcd_cnts[gcd_value];
        result %= MODULO;
    }

    return result; 
}

fn read_line_to_u64_vec() -> Vec<u64> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn main() {
    let n_and_k = read_line_to_u64_vec();
    let power = n_and_k[1];
    let numbers = read_line_to_u64_vec();

    let result = compute_sum(&numbers, power);
    println!("{result}", result=result);
}