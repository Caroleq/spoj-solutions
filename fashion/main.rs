
/// Solution to Fashion Shows Problem https://www.spoj.com/problems/FASHION/

fn read_line_to_u64_vec() -> Vec<u64> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn main() {
    let tests_count = read_line_to_usize();

    for _ in 0..tests_count {
        let _pairs_cnt = read_line_to_usize();
        let mut men = read_line_to_u64_vec();
        let mut women = read_line_to_u64_vec();
        men.sort();
        women.sort();

        let result: u64 = men.iter().zip(women.iter()).map(|(x, y)| x * y).sum();

        println!("{result}", result=result);
    }
}
