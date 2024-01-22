/// Solution to What’s Next Problem  (https://www.spoj.com/problems/ACPC10A/)

fn read_line_to_i64_vec() -> Vec<i64> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn main() {
    loop {
        let progression = read_line_to_i64_vec();
        if progression[0] == 0 && progression[1] == 0 && progression[2] == 0 {
            break;
        }

        if progression[1] - progression[0] == progression[2] - progression[1] {
            println!("AP {result}", result=2 * progression[2] - progression[1]);
        }
        else {
            println!("GP {result}", result=progression[2] * progression[2] / progression[1]);
        }
    }
}