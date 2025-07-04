/// Solution to Hotels Along the Croatian Coast (https://www.spoj.com/problems/HOTELS/)


fn read_line_to_u64_vec() -> Vec<u64> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn main() {
    let max_price_sum = read_line_to_u64_vec()[1];
    let prices = read_line_to_u64_vec();

    let mut left = 0;

    let mut window_sum = 0;
    let mut result = 0;
    for idx in 0..prices.len() {
        window_sum += prices[idx];

        while window_sum > max_price_sum {
             window_sum -= prices[left];
             left += 1;
        }

        result = std::cmp::max(result, window_sum);
    }

    println!("{result}", result=result);
}