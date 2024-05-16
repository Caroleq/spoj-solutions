// Solution to PLAY WITH MATH Problem (https://www.spoj.com/problems/ENIGMATH/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_u64_vec() -> Vec<u64> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}
fn gcd(mut n: u64, mut m: u64) -> u64 {
    while m != 0 {
        if m < n {
          std::mem::swap(&mut m, &mut n);
        }
        m %= n;
      }
      return n;
}

fn main() {
    let tests_count = read_line_to_usize();

    for _ in 0..tests_count {
        let test_case = read_line_to_u64_vec();
        let n = test_case[0];
        let m = test_case[1];

        let greatest_common_divisor = gcd(n, m);

        let x = m / greatest_common_divisor;
        let y = n / greatest_common_divisor;
        println!("{x} {y}", x=x, y=y);
    }
}