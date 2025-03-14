/// Solution to Lucifer and Magical Substrings (https://www.spoj.com/problems/MAGSUB1/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_uppercase_line_to_u8_vec() -> Vec<u8> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);

    let mut returned_string = string.trim().as_bytes().to_vec();
    for idx in 0..returned_string.len() {
        returned_string[idx] -= 65;
    }

    return returned_string;
}

static IS_PRIME: &'static [bool] = &[
    false, 
    true, // 2
    true, // 3
    false, 
    true, // 5
    false,
    true, // 7
    false,
    false,
    false, 
    true, // 11
    false,
    true, // 13
    false,
    false,
    false,
    true, // 17
    false,
    true, // 19
    false,
    false,
    false,
    true, // 23
    false,
    false,
    false];

fn main() {
    let test_cases_cnt = read_line_to_usize();

    for _ in 0..test_cases_cnt {
        let _string_size = read_line_to_usize();

        let string = read_uppercase_line_to_u8_vec();

        let mut non_prime_substr_len = 0;
        let mut non_prime_vec = Vec::new();
        for idx in 0..string.len() {
            
            if !IS_PRIME[string[idx] as usize] {
                non_prime_substr_len += 1;
            }
            else if non_prime_substr_len > 0 {
                non_prime_vec.push(non_prime_substr_len);
                non_prime_substr_len = 0;
            }
        }

        if non_prime_substr_len > 0 {
            non_prime_vec.push(non_prime_substr_len);
        }

        let str_len = string.len();
        let mut result = (str_len * str_len + str_len) / 2;
        for non_prime_len in non_prime_vec {
            result -= (non_prime_len * non_prime_len + non_prime_len) / 2;
        }

        println!("{result}", result=result);
    }
}