///  Solution to ADAROBOT Problem (https://www.spoj.com/problems/ADAROBOT/)

static BIT_CUBES: &'static [u64; 27] = &[8, 27, 64, 125, 216, 343, 512, 729, 1000, 1331, 1728, 2197, 2744, 3375, 4096, 4913, 5832, 6859, 8000, 9261, 10648, 12167, 13824, 15625, 17576, 19683, 21952];

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_lines_to_u64_vec(lines_number: usize) -> Vec<u64> {
    let mut string = String::new();
    for _ in 0..lines_number {
        let _ = std::io::stdin().read_line(&mut string);
    }

    // This may crash on Windows, because lines are separated by \r\n
    return string.trim().split("\n").map(|s| s.parse().unwrap()).collect();
}

fn compute_T(number: u64) -> u64 {
    if number % 2 == 1 {
        panic!("Only even numbers are supported!");
    }

    let mut result = 0;
    let mut power_of_2 = 2;

    let mut bit_pos = 0;
    while power_of_2 <= number {

        let power_of_2_multiples_count = 1 + (number - power_of_2) / (2 * power_of_2); 

        result += power_of_2_multiples_count * BIT_CUBES[bit_pos];
        
        power_of_2 = power_of_2 << 1;
        bit_pos += 1;
    }

    return result;
}

fn main() {
    let cases_count = read_line_to_usize();
    let vec = read_lines_to_u64_vec(cases_count);

    let mut result_string = String::new();
    for number in &vec {
        let result = compute_T(*number);
        result_string += &(result.to_string() + "\n");
    }

    print!("{}", result_string);
}