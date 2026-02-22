/// Solution to the problem Will it ever stop? (https://www.spoj.com/problems/WILLITST/)

fn read_line_to_u64() -> u64 {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn main() {
    let test_case = read_line_to_u64();
    if test_case == 1 {
        println!("NIE");
        return;
    }

    if test_case & (test_case - 1) == 0 {
        println!("TAK");
    } else {
        println!("NIE");
    }
}