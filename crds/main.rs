// Solution to Cards Problem (https://www.spoj.com/problems/CRDS/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn main() {
    let tests_count = read_line_to_usize();

    let modulo = 1000007;

    for _ in 0..tests_count {
        let test_case = read_line_to_usize();
        let result = (test_case * (test_case + 1) + test_case * (test_case - 1) / 2 ) % modulo;
        println!("{result}", result=result);
    }
}