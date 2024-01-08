// Add solution to Ambiguous Permutations problem (https://www.spoj.com/problems/PERMUT2/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_usize_vec() -> Vec<usize> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn main() {
    loop {
        let perm_size = read_line_to_usize();
        if perm_size == 0 {
            break;
        }
        let perm_numbers = read_line_to_usize_vec();

        let mut ambiguous = true;
        for idx in 0..perm_numbers.len() {
            if perm_numbers[perm_numbers[idx] - 1] != idx + 1 {
                ambiguous = false;
                break;
            }
        }

        if ambiguous {
            println!("ambiguous");
        }
        else {
            println!("not ambiguous");
        }
    }
}
