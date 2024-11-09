// Solution to Complete The Series (Easy) Problem (https://www.spoj.com/problems/AP2/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_usize_vec() -> Vec<u64> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn main() {
    let test_cases_cnt = read_line_to_usize();

    for _ in 0..test_cases_cnt {

        let test_case = read_line_to_usize_vec();
        let third_elem = test_case[0];
        let third_last_elem = test_case[1];
        let ap_sum = test_case[2];

        let elements_count = (2 * ap_sum) / (third_elem + third_last_elem);

        let common_difference = (third_last_elem - third_elem) / (elements_count - 5);

        let initial_elem = third_elem - 2 * common_difference;

        let mut element_value = initial_elem;
        println!("{elements_count}", elements_count=elements_count);
        for _ in 0..elements_count {
            print!("{elem} ", elem=element_value);
            element_value += common_difference;
        }

        println!("");
    }
}