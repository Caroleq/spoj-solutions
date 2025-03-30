/// Solution to Running Median Again (https://www.spoj.com/problems/RMID2/)

use std::collections::BinaryHeap;

fn read_line_to_i64() -> i64 {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an i64 integer");
}

fn main() {
    let test_cases_cnt = read_line_to_i64();

    let mut result_str = String::new();

    for _ in 0..test_cases_cnt {
        let mut below_or_eq_median = BinaryHeap::new();
        let mut above_median = BinaryHeap::new();

        loop {
            let number = read_line_to_i64();
            if number == 0 {
                break;
            }
            else if number == -1 {
                let result: i64 = below_or_eq_median.pop().unwrap();
                result_str += &result.to_string();
                result_str += "\n";

                if below_or_eq_median.len() < above_median.len() {
                    below_or_eq_median.push(above_median.pop().unwrap() * (-1));   
                }
            }
            else {
                if below_or_eq_median.is_empty() {
                    below_or_eq_median.push(number);
                    continue;
                }

                if above_median.is_empty() {
                    below_or_eq_median.push(number);
                    above_median.push(below_or_eq_median.pop().unwrap() * (-1));
                    continue;
                }
                let above_median_min_elem = above_median.peek().unwrap() * (-1);
                if number <= above_median_min_elem {
                    below_or_eq_median.push(number);
                    if below_or_eq_median.len() > above_median.len() + 1 {
                        above_median.push(below_or_eq_median.pop().unwrap() * (-1));
                    }
                }
                else {
                    above_median.push(number * (-1));
                    if below_or_eq_median.len() < above_median.len() {
                        below_or_eq_median.push(above_median.pop().unwrap() * (-1));   
                    }
                }
            }
        }
    }
    print!("{result}", result=result_str);
}