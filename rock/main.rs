// Solution to Sweet and Sour Rock problem (https://www.spoj.com/problems/ROCK/)

fn read_line_to_size() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_u8_vec() -> Vec<u8> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().as_bytes().to_vec();
}

fn compute_max_gain_for_rock(rock_segments: &Vec<u8> ) -> u32 {

    let rock_length = rock_segments.len();

    let mut dp = vec![vec![0; rock_length]; rock_length];
    let mut max_gains = vec![0; rock_length];

    let zero_byte = 48;
    let one_byte = 49;

    if rock_segments[0] == one_byte {
        dp[0][0] = 1;
        max_gains[0] = 1;
    }

    for segment_idx in 1..rock_length {

        let mut sum_ones = 0;
        let mut sum_zeros = 0;

        dp[segment_idx][0] = max_gains[segment_idx - 1];
        max_gains[segment_idx] = max_gains[segment_idx - 1];
        if rock_segments[segment_idx] == one_byte {
            dp[segment_idx][0] += 1;
            max_gains[segment_idx] += 1;
            sum_ones += 1;
        }
        else {
            sum_zeros += 1;
        }

        for segment_len in 1..segment_idx + 1 {
            if rock_segments[segment_idx - segment_len] == zero_byte {
                sum_zeros += 1;
            }
            else {
                sum_ones += 1;
            }

            if segment_idx > segment_len {
                dp[segment_idx][segment_len] = max_gains[segment_idx - segment_len - 1];
            }

            if sum_ones > sum_zeros {
                dp[segment_idx][segment_len] += segment_len + 1;
                max_gains[segment_idx] = std::cmp::max(max_gains[segment_idx], dp[segment_idx][segment_len]);
            }
        }
    }

    return max_gains[rock_length - 1] as u32;
}

fn main() {

    let cases_cnt = read_line_to_size();
    for _ in 0..cases_cnt {
        let _ = read_line_to_size();
        let rock_segments = read_line_to_u8_vec();

        let result = compute_max_gain_for_rock(&rock_segments);
        println!("{result}", result=result);
    }
}
