/// Solution to Penney Game (https://www.spoj.com/problems/NY10A/)
use std::collections::HashMap;

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_string() -> String {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().to_string();
}

fn main() {
    let test_case_cnt = read_line_to_usize();
    for _ in 0..test_case_cnt {
        let case_id = read_line_to_usize();
        let penny_values = read_line_to_string();

        let mut frequencies = HashMap::from([
            ("TTT", 0),
            ("TTH", 0),
            ("THT", 0),
            ("THH", 0),
            ("HTT", 0),
            ("HTH", 0),
            ("HHT", 0),
            ("HHH", 0),
        ]);

        for idx in 0..penny_values.len() - 2 {
            *frequencies.get_mut(&penny_values[idx..idx + 3]).unwrap() += 1;
        }

        println!("{case_idx} {TTT} {TTH} {THT} {THH} {HTT} {HTH} {HHT} {HHH}", 
                    case_idx=case_id,
                    TTT=frequencies["TTT"], 
                    TTH=frequencies["TTH"],
                    THT=frequencies["THT"],
                    THH=frequencies["THH"],
                    HTT=frequencies["HTT"],
                    HTH=frequencies["HTH"],
                    HHT=frequencies["HHT"],
                    HHH=frequencies["HHH"]
                    );
    }

}