/// Solution to Advanced Fruits Problem (https://www.spoj.com/problems/ADFRUITS/)

fn compute_combination(a_fruit: &Vec<u8>, b_fruit: &Vec<u8>) -> Vec<u8> {

    let mut lcs = vec![vec![0; b_fruit.len() + 1]; a_fruit.len() + 1];

    for a_idx in 0..a_fruit.len() {
        for b_idx in 0..b_fruit.len() {
            if a_fruit[a_idx] == b_fruit[b_idx] {
                lcs[a_idx + 1][b_idx + 1] = lcs[a_idx][b_idx] + 1;
            }
            else {
                lcs[a_idx + 1][b_idx + 1] = std::cmp::max(lcs[a_idx + 1][b_idx], lcs[a_idx][b_idx + 1]);
            }
        }
    }

    let mut a_idx = a_fruit.len();
    let mut b_idx = b_fruit.len();

    let mut result_fruit_rev = Vec::new();
    while a_idx > 0 && b_idx > 0 {
        if a_fruit[a_idx - 1] == b_fruit[b_idx - 1] {
            result_fruit_rev.push(a_fruit[a_idx - 1]);
            a_idx -= 1;
            b_idx -= 1;
        }
        else if lcs[a_idx - 1][b_idx ] > lcs[a_idx][b_idx - 1] {
            result_fruit_rev.push(a_fruit[a_idx - 1]);
            a_idx -= 1;
        } else {
            result_fruit_rev.push(b_fruit[b_idx - 1]);
            b_idx -= 1;
        }
    }

    while a_idx > 0 {
        result_fruit_rev.push(a_fruit[a_idx - 1]);
        a_idx -= 1;
    }

    while b_idx > 0 {
        result_fruit_rev.push(b_fruit[b_idx - 1]);
        b_idx -= 1;
    }

    return result_fruit_rev.into_iter().rev().collect();
}

fn main() {
    loop {
        let mut string = String::new();
        let string_length = std::io::stdin().read_line(&mut string).expect("Could not parse problem size");
        if string_length == 0 || string.trim().len() == 0 {
            break;
        }

        let fruits: Vec<String> = string.trim().split(' ').map(|s| s.parse().unwrap()).collect();

        let a_fruit = fruits[0].trim().as_bytes().to_vec();
        let b_fruit = fruits[1].trim().as_bytes().to_vec();
        let result_fruit = compute_combination(&a_fruit, &b_fruit);

        let result_fruit_str = match std::str::from_utf8(&result_fruit) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        println!("{result_fruit_str}", result_fruit_str=result_fruit_str);
    }
}