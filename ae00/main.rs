// Solution to Rectangles Problem (https://www.spoj.com/problems/AE00/)

fn read_line_to_u64() -> u64 {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn main() {
    let squares_cnt = read_line_to_u64();

    let mut result = 0;
    let mut side_length = 1;
    while side_length * side_length <= squares_cnt {
        result += squares_cnt / side_length - (side_length - 1);
        side_length += 1;
    }

    println!("{result}", result=result);
}