/// Solution to Adding Reversed Numbers (https://www.spoj.com/problems/ADDREV/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_string_vec() -> Vec<String> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn main() {
    let cases_count = read_line_to_usize(); 

    for _ in 0..cases_count {

        let numbers = read_line_to_string_vec();

        let number1_rev: u32 = numbers[0].chars().rev().collect::<String>().parse().unwrap();
        let number2_rev: u32 = numbers[1].chars().rev().collect::<String>().parse().unwrap();
        let mut result_rev =  number1_rev + number2_rev;
        while result_rev % 10 == 0 {
            result_rev = result_rev / 10;
        }
        let result_str: String = result_rev.to_string().chars().rev().collect();

        println!("{result_str}", result_str=result_str);
    }
}