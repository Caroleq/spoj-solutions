///  Solution to EKO Problem (https://www.spoj.com/problems/EKO/)

fn read_line_to_u64_vec() -> Vec<u64> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn main() {
    let m = read_line_to_u64_vec()[1];
    let tree_heights = read_line_to_u64_vec();

    let mut lower_bound = 0;
    let mut upper_bound = *tree_heights.iter().max().expect("empty vector");

    while lower_bound + 1 < upper_bound {
        let mid_value = (lower_bound + upper_bound) / 2;

        let mut sum = 0;
        for height in tree_heights.iter() {
            if *height > mid_value {
                sum += *height - mid_value;
            }
        }

        if sum >= m {
            lower_bound = mid_value;
        }
        else {
            upper_bound = mid_value;
        }
    }

    let mut sum = 0;
    for height in tree_heights.iter() {
        if *height > upper_bound {
            sum += *height - upper_bound;
        }
    }

    if sum >= m {
        println!("{upper_bound}", upper_bound=upper_bound);
    }
    else {
        println!("{lower_bound}", lower_bound=lower_bound);
    }
}