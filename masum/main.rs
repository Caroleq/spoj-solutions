/// Solution to Maximum Sum of the Array Problem (https://www.spoj.com/problems/MASUM/)

fn read_line() -> String {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string;
}

fn read_line_to_i64_vec() -> Vec<i64> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
 }

 fn main() {
    let _ = read_line();
    let sequence = read_line_to_i64_vec();
    let mut max_sum = sequence.iter().fold(0u64, |sum, val| sum + (val.abs() as u64));
    let min_value = *sequence.iter().min().unwrap();
    let max_value = *sequence.iter().max().unwrap();
    if min_value > 0 {
        max_sum -= 2 * (min_value as u64);
    }
    else if max_value < 0 {
        max_sum -= 2 * (max_value.abs() as u64);
    }
    println!("{max_sum}", max_sum=max_sum);
}