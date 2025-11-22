/// Solution to Power Tower City (https://www.spoj.com/problems/POWTOW/)
/// Implementation based on:
/// https://stackoverflow.com/questions/30713648/how-to-compute-ab-mod-m

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an usize integer");
}

fn read_line_to_u64_vec() -> Vec<u64> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

static LAMBDA: [u64; 12] = [ 1000000000, 50000000, 2500000, 125000, 12500, 2500, 500, 100, 20, 4, 2, 1];

fn compute_log_2_ceil(number: u64) -> u64 {
    let mut exponent = 0;
    let mut power = 1;
    while power < number {
        power *= 2;
        exponent += 1;
    }
    return exponent;
}

fn is_greater_than_log_2(base: u64, exponents_cnt: u64, index: usize) -> bool {
    if exponents_cnt == 0 {
        return LAMBDA[index] == 1;
    }

    let log2 = compute_log_2_ceil(LAMBDA[index]);

    if log2 <= base {
        return true;
    }

    if base >= 10 {
        return true;
    }

    let mut exponents_cnt = exponents_cnt;
    let mut result = base as u128;
    while exponents_cnt > 1 && result < log2 as u128 {
        result = result.pow(base as u32);
        exponents_cnt -= 1;
    }

    if log2 as u128 <= result {
        return true;
    }
    
    return false;
}

fn compute_exponent(number: u64, exponent: u64, modulo: u64) -> u64 {
    let mut exponent = exponent;

    if exponent == 0 {
        return 1;
    }

    let mut result = number;
    let mut number = number;
    exponent -= 1;

    while exponent > 0 {

        if (exponent % 2) == 1 {
            result *= number;
        }

        result %= modulo as u64;

        number *= number;
        number %= modulo;
        exponent /= 2;
    }

    return result as u64;
}

fn direct_compute(base: u64, exponents_cnt: u64) -> u64 {
    if exponents_cnt == 0 {
        return 1;
    }
    let mut result = base;
    let mut exponents_cnt = exponents_cnt;
    while exponents_cnt > 1 {
        result = result.pow(base as u32);
        exponents_cnt -= 1;
    }
    return result;
}

fn compute(base: u64, exponents_cnt: u64, index: usize) -> u64 {
    if exponents_cnt == 0 {
        return 1;
    }

    if base % LAMBDA[index] == 0 {
        return 0;
    }

    if index + 1 == LAMBDA.len() - 1 {
        return base;
    }

    if !is_greater_than_log_2(base, exponents_cnt, index) {
        return direct_compute(base, exponents_cnt) % LAMBDA[index];
    }
    let result = compute(base, exponents_cnt - 1, index + 1);
    if result == 0 {
        return compute_exponent(base, result + LAMBDA[index + 1], LAMBDA[index]) % LAMBDA[index]; 
    }
    return compute_exponent(base, result, LAMBDA[index]) % LAMBDA[index];
}

fn compute_power_tower(base: u64, exponents_cnt: u64) -> u64 {
    if base == 1 {
        return 1;
    }

    if base == 0 {
        if exponents_cnt == 0{
            return 1;
        }
        
        if exponents_cnt % 2 == 0 {
            return 1;
        }

        return 0;
    }

    return compute(base, exponents_cnt, 0) % LAMBDA[0];
}

fn add_dots(base: u64, exponents_cnt: u64) -> bool {
    if base == 0 || base == 1 {
        return false;
    }

    if exponents_cnt == 0 {
        return false;
    }

    if base == 2 {
        return exponents_cnt > 4;
    }

    if base >= 3 && base <= 9 {
        return exponents_cnt > 2; 
    }

    if exponents_cnt == 1 {
        return base >= LAMBDA[0];
    }

    return true;
}

fn main() {
   let test_case_cnt = read_line_to_usize();
   let mut all_results = String::new();
   for _ in 0..test_case_cnt {

    let test_case = read_line_to_u64_vec();

    let result = compute_power_tower(test_case[0], test_case[1]);
    if add_dots(test_case[0], test_case[1]) {
       all_results += "...";
       let repeat = "0".repeat(9 - result.to_string().len());
       all_results += &repeat;
       all_results += &result.to_string();
       all_results += "\n";
    }
    else {
        all_results += &result.to_string();
        all_results += "\n";
    }
   }
   print!("{result}", result=all_results);
}