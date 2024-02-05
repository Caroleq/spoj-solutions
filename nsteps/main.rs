// Solution to Number Steps Problem (https://www.spoj.com/problems/NSTEPS/)

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

fn main() {
    let cases_count = read_line_to_usize();

    for _ in 0..cases_count {
        let vector = read_line_to_i64_vec();
        if vector[0] == vector[1] {
            if vector[0] % 2 == 0 {
                println!("{result}", result=2 * vector[0]);
            }
            else {
                println!("{result}", result=2 * vector[0] - 1);
            }
        }
        else if vector[0] - 2 == vector[1] {
            if vector[0] % 2 == 0 {
                println!("{result}", result=vector[0] + vector[1]);
            }
            else {
                println!("{result}", result=vector[0] + vector[1] - 1);
            }
        }
        else {
            println!("No Number");
        }
    }
}