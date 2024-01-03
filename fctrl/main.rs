/// Solution to Factorial problem (https://www.spoj.com/problems/FCTRL/)

fn read_line_to_u64() -> u64 {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn compute_z_function(number: u64) -> u64 {
    let mut result = 0;
    let mut number = number;
    while number > 0 {
        result += number / 5;
        number /= 5;
    }
    return result;
}

fn main() {

    let cases_cnt = read_line_to_u64();
    for _ in 0..cases_cnt {
        let number = read_line_to_u64();
        let z_function_value = compute_z_function(number);
        println!("{result}", result=z_function_value);
    }
}