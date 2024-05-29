/// Solution to Greatest Common Divisor Of Three Integers (https://www.spoj.com/problems/GDCOFTI/)

fn read_line_to_u64() -> u64 {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
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

    let first_number = read_line_to_u64();
    let second_number = read_line_to_u64();
    let third_number = read_line_to_u64();

    let result = gcd(gcd(first_number, second_number), third_number);
    println!("{result}", result=result);
}