// Solution to Stamps Problem (https://www.spoj.com/problems/STAMPS/)

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
    let test_cases_cnt = read_line_to_usize();

    for case_idx in 0..test_cases_cnt {
        let first_line_of_case = read_line_to_usize_vec();
        let target_stamps_cnt = first_line_of_case[0];
        let mut stamps = read_line_to_usize_vec();
        stamps.sort();
        stamps.reverse();

        let mut gathered_stamps = 0;
        let mut friends_cnt = 0;
        for stamp_cnt in stamps {
            gathered_stamps += stamp_cnt;
            friends_cnt += 1;

            if gathered_stamps >= target_stamps_cnt {
                break;
            }
        }

        println!("Scenario #{case_idx}:", case_idx=case_idx + 1);
        if gathered_stamps >= target_stamps_cnt {
            println!("{friends_cnt}", friends_cnt=friends_cnt);
        }
        else {
            println!("impossible");
        }
        println!("");
    }
}