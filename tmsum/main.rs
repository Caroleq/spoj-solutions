// Solution to The Maximize Sum Problem (https://www.spoj.com/problems/TMSUM/)


fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_i64_vec() -> Vec<i64> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn compute_max_sum(numbers: &Vec<i64>) -> i64 {
    let mut numbers = numbers.clone();
    numbers.sort();

    let mut sum = 0;

    let mut idx = 0;
    while idx < numbers.len() - 1 && numbers[idx + 1] < 0 { 
        sum += numbers[idx] * numbers[idx + 1];
        idx += 2;
    }

    if idx == numbers.len() - 1 {
        sum += numbers[idx];
        return sum;
    }

    if numbers[idx] < 0 {
        if numbers[idx + 1] != 0 {
            sum += numbers[idx];
            idx += 1;
        }
        else {
            idx += 2;
        }
    }

    while idx < numbers.len() && numbers[idx] < 2 { 
        sum += numbers[idx];
        idx += 1;
    }

    let even_number_of_elems_left = (numbers.len() - idx) % 2 == 0;
    if !even_number_of_elems_left {
        sum += numbers[idx];
        idx += 1;
    }

    while idx < numbers.len() {
        sum += numbers[idx] * numbers[idx + 1];
        idx += 2;
    }

    return sum;
}

fn main() {

    let tests_count = read_line_to_usize();

    for _ in 0..tests_count {

        let _ = read_line_to_usize();
        let numbers = read_line_to_i64_vec();

        let result = compute_max_sum(&numbers);
        println!("{result}", result=result);

    }
}