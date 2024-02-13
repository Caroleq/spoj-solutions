// Solution to Colored Development Problem (https://www.spoj.com/problems/BANSTAND/)

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

fn gcd(mut n: u64, mut m: u64) -> u64 {
    while m != 0 {
        if m < n {
          std::mem::swap(&mut m, &mut n);
        }
        m %= n;
      }
      return n;
}

fn fast_exp(number: u64, exp_value: u64, modulo: u64) -> u64 {
    if exp_value == 0 {
        return 1;
    }
    if exp_value == 1 {
        return number % modulo;
    }

    if exp_value % 2 == 0 {
        return fast_exp((number * number) % modulo, exp_value / 2, modulo);
    }

    return (number * fast_exp((number * number) % modulo, exp_value / 2, modulo)) % modulo;
}

// Function based on implementation from https://cp-algorithms.com/algebra/extended-euclid-algorithm.html
fn extended_euclidean(a: i64, b: i64, x: & mut i64, y: &mut i64) -> i64 {
    if b == 0 {
        *x = 1;
        *y = 0;
        return a;
    }
    let mut x1: i64 = 0;
    let mut y1: i64 = 0;
    let d = extended_euclidean(b, a % b, &mut x1, &mut y1);
    *x = y1;
    *y = x1 - y1 * (a / b);
    return d;
}


fn main() {
    let tests_count = read_line_to_usize();

    let modulo = 1000000007;

    for test_id in 0..tests_count {
        let test_case = read_line_to_u64_vec();
        let n = test_case[0];
        let m = test_case[1];

        let n_exp_m_sub_1 = fast_exp(n, m - 1, modulo);
        let n_exp_m = (n_exp_m_sub_1 * n) % modulo;
        let n_sub_1_exp_m = fast_exp(n - 1, m, modulo);

        let nominator = (modulo + n_exp_m - n_sub_1_exp_m) % modulo;
        let denominator = n_exp_m_sub_1;

        let gcd_value = gcd(nominator, denominator);
        let nominator = nominator / gcd_value;
        let denominator = denominator / gcd_value;

        let mut x1: i64 = 0;
        let mut y1: i64 = 0;
        let _ = extended_euclidean(denominator as i64, modulo as i64, &mut x1, & mut y1);
        let inv_denominator: u64  = ((x1 % (modulo as i64) + (modulo as i64)) as u64) % modulo as u64;

        let result = (inv_denominator * nominator) % modulo;
        println!("Case {case_no}: {result}", case_no=(test_id + 1), result = result);
    }

}