// Add solution to Ada and Digits problem (https://www.spoj.com/problems/ADADIG/)
use std::collections::HashMap;

fn read_line_to_u64() -> u64 {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn factorize(number: u64) -> (HashMap<u64, u64>, u64) {

    let mut number = number;

    let mut factors = HashMap::new();
    factors.insert(2, 0);
    factors.insert(3, 0);
    factors.insert(5, 0);
    factors.insert(7, 0);

    while number % 2 == 0 {
        *factors.get_mut(&2).unwrap() += 1;
        number /= 2;
    }

    while number % 3 == 0 {
        *factors.get_mut(&3).unwrap() += 1;
        number /= 3;
    }

    while number % 5 == 0 {
        *factors.get_mut(&5).unwrap() += 1;
        number /= 5;
    }

    while number % 7 == 0 {
        *factors.get_mut(&7).unwrap() += 1;
        number /= 7;
    }  

    return (factors, number);
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

fn choose(n: u64, k: u64, modulo: u64) -> u64 {

    if k == n || k == 0 {
        return 1;
    }

    let mut k_factorial = 1;
    let mut tmp = k;
    while tmp != 0 {
        // task constraints guarantee that k! will fit in u64.
        k_factorial = k_factorial * tmp;
        tmp -= 1;
    }

    let mut multiply_elements: Vec<u64> = (n - k + 1..n + 1).collect();

    for elem_idx in 0..multiply_elements.len() {

        let divide_factor = gcd(multiply_elements[elem_idx], k_factorial);
        multiply_elements[elem_idx] /= divide_factor;
        k_factorial /= divide_factor;

        if k_factorial == 1 {
            break;
        }
    }

    let mut return_value = 1;
    for elem in multiply_elements {
        return_value = (return_value * elem) % modulo;
    }
    return return_value;
}

fn compute_puzzle(number: u64) -> u64 {

    if number == 1 {
        return 1;
    }

    let (factors, rest) = factorize(number);

    if rest != 1 {
        return 0;
    }

    let modulo = 1000000007;

    let init_two_cnt = *factors.get(&2).unwrap();
    let init_three_cnt = *factors.get(&3).unwrap();
    let five_cnt = *factors.get(&5).unwrap();
    let seven_cnt = *factors.get(&7).unwrap();

    let mut result = 0;

    let max_four_cnt = init_two_cnt / 2;
    for four_cnt in 0..max_four_cnt + 1 {

        let max_six_cnt = std::cmp::min(&init_two_cnt - 2 * four_cnt, init_three_cnt);
        for six_cnt in 0..max_six_cnt + 1 {

            let max_eight_cnt = (init_two_cnt - 2 * four_cnt - six_cnt) / 3;
            for eight_cnt in 0..max_eight_cnt + 1 {

                let max_nine_cnt = (init_three_cnt - six_cnt) / 2;
                for nine_cnt in 0..max_nine_cnt + 1 {

                    let two_cnt = init_two_cnt - four_cnt * 2 - six_cnt - eight_cnt * 3;
                    let three_cnt = init_three_cnt - six_cnt - nine_cnt * 2;

                    let digits_sum_but_one = two_cnt * 2 + three_cnt * 3 + four_cnt * 4 + five_cnt * 5 
                            + six_cnt * 6 + seven_cnt * 7 + eight_cnt * 8 + nine_cnt * 9;
                    let one_cnt = number - digits_sum_but_one;

                    let digits_cnt = one_cnt + two_cnt + three_cnt + four_cnt + five_cnt + 
                                            six_cnt + seven_cnt + eight_cnt + nine_cnt;

                    let mut digits_left = digits_cnt; 

                    let mut case_selection = 1; 
                    
                    let two_selection = choose(digits_left, two_cnt, modulo);

                    digits_left -= two_cnt;
                    case_selection = (case_selection * two_selection) % modulo;

                    let three_selection = choose(digits_left, three_cnt, modulo);
                    digits_left -= three_cnt;
                    case_selection = (case_selection * three_selection) % modulo;

                    let four_selection = choose(digits_left, four_cnt, modulo);
                    digits_left -= four_cnt;
                    case_selection = (case_selection * four_selection) % modulo;

                    let five_selection = choose(digits_left, five_cnt, modulo);
                    digits_left -= five_cnt;
                    case_selection = (case_selection * five_selection) % modulo;

                    let six_selection = choose(digits_left, six_cnt, modulo);
                    digits_left -= six_cnt;
                    case_selection = (case_selection * six_selection) % modulo;

                    let seven_selection = choose(digits_left, seven_cnt, modulo);
                    digits_left -= seven_cnt;
                    case_selection = (case_selection * seven_selection) % modulo;

                    let eight_selection = choose(digits_left, eight_cnt, modulo);
                    digits_left -= eight_cnt;
                    case_selection = (case_selection * eight_selection) % modulo;

                    let nine_selection = choose(digits_left, nine_cnt, modulo);
                    case_selection = (case_selection * nine_selection) % modulo;

                    result = (result + case_selection) % modulo;

                }
            }
        }
    }

    return result;
}

fn main() {
    let test_cases_cnt = read_line_to_u64();

    for _ in 0..test_cases_cnt {
        let number = read_line_to_u64();

        let result = compute_puzzle(number);
        println!("{result}", result=result);
    }
}