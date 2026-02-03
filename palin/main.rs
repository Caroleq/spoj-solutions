/// Solution to the "Next Palindrome" problem on SPOJ (https://www.spoj.com/problems/PALIN/)


fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_u8_vec() -> Vec<u8> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().as_bytes().to_vec();
}

fn first_non_nine_from_the_middle(number: &Vec<u8>) -> isize {

    let middle = match number.len() % 2 {
        0 => number.len() / 2 - 1,
        1 => number.len() / 2,
        _ => 0,
    };
    
    for idx in (0..middle + 1).rev() {
        if number[idx] != 57 {
            return idx as isize;
        }
    }

    return -1;
}

fn will_making_palindrome_increase_number(number: &Vec<u8>) -> bool {
    let middle = match number.len() % 2 {
        0 => number.len() / 2 - 1,
        1 => number.len() / 2,
        _ => 0,
    };
    
    for idx in (0..middle + 1).rev() {
        let left_char = number[idx];
        let right_char = number[number.len() - idx - 1];

        if left_char < right_char {
            return false;
        }
        else if left_char > right_char {
            return true;
        }
    }

    return false;
}

fn find_next_palindrome(number: &Vec<u8>) -> Vec<u8> {

    if will_making_palindrome_increase_number(number) {
        let mut result = number.clone();
        for idx in 0..(number.len() / 2) {
            result[number.len() - idx - 1] = result[idx];
        }
        return result;
    }
    
    let first_non_nine = first_non_nine_from_the_middle(&number);
    if first_non_nine == -1 {
        let mut new_result = vec![48; number.len() + 1];
        new_result[0] = 49;
        new_result[number.len()] = 49;
        return new_result;
    }

    let mut result = number.clone();

    for idx in 0..first_non_nine as usize {
        result[number.len() - idx - 1] = result[idx];
    }

    result[first_non_nine as usize] += 1;
    result[number.len() - first_non_nine as usize - 1] = result[first_non_nine as usize];

    for idx in (first_non_nine as usize + 1)..(number.len() - first_non_nine as usize - 1) {
        result[idx] = 48;
    }

    return result;
}

fn main() {
    let tests_count = read_line_to_usize();

    for _ in 0..tests_count {
        let test_case = read_line_to_u8_vec();
        let next_palindrome = find_next_palindrome(&test_case);
        println!("{}", String::from_utf8(next_palindrome).unwrap());
    }
}