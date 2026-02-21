///Solution to Count On Cantor Problem (https://www.spoj.com/problems/CANTON/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn compute_fraction(number: usize) -> (usize, usize) {
    let mut row = ((-1.0 + (1.0 + 8.0 * number as f64).sqrt()) / 2.0) as usize;
    if row * (row + 1) / 2 == number {
        let from_top_right_to_bottom_left = row % 2 == 0;
        if from_top_right_to_bottom_left {
            return (row, 1);
        } else {
            return (1, row);
        }
    }
    row += 1;

    let distance =  row * (row + 1) / 2 - number;

    let from_top_right_to_bottom_left = row % 2 == 0;
    if from_top_right_to_bottom_left {
        return (row - distance, distance + 1);
    } else {
        return (distance + 1, row - distance);
    }
}

fn main() {
    let tests_count = read_line_to_usize();

    for _ in 0..tests_count {
        let number = read_line_to_usize();
        let (numerator, denominator) = compute_fraction(number);
        println!("TERM {number} IS {numerator}/{denominator}", number=number, numerator=numerator, denominator=denominator);
    }
}