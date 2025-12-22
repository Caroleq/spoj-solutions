/// Solution to Modular Tetration (https://www.spoj.com/problems/MTETRA/)
/// Implementation based on:
/// https://stackoverflow.com/questions/30713648/how-to-compute-ab-mod-m
/// https://cp-algorithms.com/algebra/phi-function.html

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

fn compute_phi(number: u64) -> u64 {
    let first_phi = vec![0, 1, 1, 2, 2, 4, 2, 6, 4, 6, 4];
    if number < 11 {
        return first_phi[number as usize];
    }
    let mut result = number;
    let mut i = 2;
    let mut number = number;
    while i * i <= number {
      
      while (number % i) == 0 {
        while (number % i) == 0 {
            number /= i;
        }            
        result -= result / i;
      }
      i += 1;
    }

    if number > 1 {
        result -= result / number;
    }
    return result;
}


fn compute_log_2_ceil(number: u64) -> u64 {
    let mut exponent = 0;
    let mut power = 1;
    while power < number {
        power *= 2;
        exponent += 1;
    }
    return exponent;
}

fn is_greater_than_log_2(base: u64, exponents_cnt: u64, phi: u64) -> bool {
    if exponents_cnt == 0 {
        return phi == 1;
    }

    let log2 = compute_log_2_ceil(phi);

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

fn compute(base: u64, exponents_cnt: u64, modulo: u64) -> u64 {
    if exponents_cnt == 0 {
        return 1;
    }

    if base % modulo == 0 {
        return 0;
    }

    if modulo == 1 {
        return base;
    }

    if !is_greater_than_log_2(base, exponents_cnt, modulo) {
        return direct_compute(base, exponents_cnt) % modulo;
    }
    
    let result = compute(base, exponents_cnt - 1, compute_phi(modulo));
    if result == 0 {
         return compute_exponent(base, result + compute_phi(modulo), modulo) % modulo; 
    }
    return compute_exponent(base, result, modulo);
}

fn compute_power_tower(base: u64, exponents_cnt: u64, modulo: u64) -> u64 {
    if base == 1 {
        return 1;
    }

    if base == 0 {
        if exponents_cnt == 0 {
            return 1;
        }
        
        if exponents_cnt % 2 == 0 {
            return 1;
        }

        return 0;
    }

    return compute(base, exponents_cnt, modulo) % modulo;
}

fn main() {
   let test_case_cnt = read_line_to_usize();
   let mut all_results = String::new();
   for _ in 0..test_case_cnt {

    let test_case = read_line_to_u64_vec();

    let result = compute_power_tower(test_case[0], test_case[1], test_case[2]);
    all_results += &result.to_string();
    all_results += "\n";

   }
   print!("{result}", result=all_results);
}