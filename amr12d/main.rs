/// Solution to The Mirror of Galadriel problem (https://www.spoj.com/problems/AMR12D/)


fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line() -> String {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().to_string(); 
}

fn main() {
    let test_case_cnt = read_line_to_usize();

    for _ in 0..test_case_cnt {
        let string = read_line();
        if string == string.chars().rev().collect::<String>() {
            println!("YES");
        }
        else {
            println!("NO");
        }
    }
}