///  Solution to Candy I Problem (https://www.spoj.com/problems/CANDY/)

fn read_line_to_i32() -> i32 {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_lines_to_u32_vec(lines_number: usize) -> Vec<u32> {
    let mut result = Vec::new();
    for _ in 0..lines_number {
        result.push(read_line_to_i32() as u32);
    }

    return result;
}

fn compute_average(candy_counts: &Vec<u32>) -> i32 {
    let candy_sum: u32 = candy_counts.iter().sum();
    let avg_rounded = candy_sum / candy_counts.len() as u32;
    if avg_rounded * candy_counts.len() as u32 != candy_sum {
        return -1;
    }

    let mut moves_count = 0;
    for candy_count in candy_counts {
        if candy_count > &avg_rounded {
            moves_count += candy_count - avg_rounded;
        }
    }

    return moves_count as i32;
}

fn main() {

    loop {
        let case_size = read_line_to_i32();
        if case_size == -1 {
            break;
        }

        let candy_counts = read_lines_to_u32_vec(case_size as usize);
        let moves_count = compute_average(&candy_counts);

        println!("{moves_count}", moves_count=moves_count);
    }

}