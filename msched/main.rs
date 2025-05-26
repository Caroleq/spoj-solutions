use std::collections::BinaryHeap;

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_i64_vec() -> Vec<i64> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}


fn main() {

    let cows_cnt = read_line_to_usize();
    
    let mut cows_params = Vec::new();

    for _ in 0..cows_cnt {
        let cow_params = read_line_to_i64_vec();
        cows_params.push((cow_params[0], cow_params[1]));
    }

    cows_params.sort_by_key(|params| params.1);

    let mut cows = BinaryHeap::new();
    for params in cows_params {
        if cows.len() < params.1 as usize {
            cows.push((-1) * params.0);
        }
        else if cows.peek().unwrap() * (-1) < params.0 {
            cows.pop();
            cows.push((-1) * params.0);
        }
    }

    let mut max_profit = 0;
    for profit in cows {
        max_profit += (-1) * profit;
    }

    println!("{max_profit}", max_profit=max_profit);

}