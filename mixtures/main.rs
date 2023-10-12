///  Solution to MIXTURES Problem (https://www.spoj.com/problems/MIXTURES/)

fn read_line_to_u8_vec() -> Vec<u8> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn compute_min_smoke(mixtures: &Vec<u8>, l_idx: usize, r_idx: usize, smoke_n_color: &mut Vec<Vec<(i64, u8)>>) {

    if l_idx >= r_idx {
        panic!("Invalid program state.");
    }

    if smoke_n_color[l_idx][r_idx].0 > -1 {
        return;
    }

    if l_idx + 1 == r_idx {
        smoke_n_color[l_idx][r_idx] = ((mixtures[l_idx] as i64) * (mixtures[r_idx] as i64), (mixtures[l_idx] + mixtures[r_idx]) % 100);
        return;
    }

    compute_min_smoke(mixtures, l_idx + 1, r_idx, smoke_n_color);
    let mut min_smoke = (smoke_n_color[l_idx + 1][r_idx].0 + (smoke_n_color[l_idx + 1][r_idx].1 as i64) * (mixtures[l_idx] as i64)) as u64;

    compute_min_smoke(mixtures, l_idx, r_idx - 1, smoke_n_color);
    let min_smoke_all_but_rightmost = (smoke_n_color[l_idx][r_idx - 1].0 + (smoke_n_color[l_idx][r_idx - 1].1 as i64) * (mixtures[r_idx] as i64)) as u64;
    if min_smoke_all_but_rightmost < min_smoke {
        min_smoke = min_smoke_all_but_rightmost;
    }

    for idx in l_idx + 1..r_idx - 1 {
        compute_min_smoke(mixtures, l_idx, idx, smoke_n_color);
        compute_min_smoke(mixtures, idx + 1, r_idx, smoke_n_color);

        let left_result = smoke_n_color[l_idx][idx];
        let right_result = smoke_n_color[idx + 1][r_idx];
        
        let combined_smoke = (left_result.0 + right_result.0 + (left_result.1 as i64 ) * (right_result.1 as i64)) as u64;
        if combined_smoke < min_smoke {
            min_smoke = combined_smoke;
        }
    }

    smoke_n_color[l_idx][r_idx] = (min_smoke as i64, (smoke_n_color[l_idx + 1][r_idx].1 + mixtures[l_idx]) % 100);
}

fn main() {
    loop {
        let mut string = String::new();
        let string_length = std::io::stdin().read_line(&mut string).expect("Could not parse problem size");
        if string_length == 0 || string.trim().len() == 0 {
            break;
        }

        let mixtures = read_line_to_u8_vec();
        let problem_size = mixtures.len();
        if problem_size == 1 {
            println!("0");
            continue;
        }

        let mut smoke_n_color = vec![vec![(-1, 0); problem_size]; problem_size];
        compute_min_smoke(&mixtures, 0, problem_size - 1, &mut smoke_n_color);
        println!("{min_smoke}", min_smoke=smoke_n_color[0][problem_size - 1].0);

    }
}