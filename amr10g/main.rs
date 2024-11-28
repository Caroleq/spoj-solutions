/// Solution to Christmas Play problem (https://www.spoj.com/problems/AMR10G/)

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
    let test_case_cnt = read_line_to_usize();
    for _ in 0..test_case_cnt {
        let test_case_first_line = read_line_to_usize_vec();
        let searched_kids_cnt = test_case_first_line[1];

        let mut heights = read_line_to_usize_vec(); 
        heights.sort();

        let mut minimum_difference = usize::max_value();
        for idx in searched_kids_cnt - 1..heights.len() {
            minimum_difference = std::cmp::min(minimum_difference, heights[idx] - heights[(idx + 1) - searched_kids_cnt]);
        }

        println!("{minimum_difference}", minimum_difference=minimum_difference);
    }
}