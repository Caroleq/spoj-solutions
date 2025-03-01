/// Solution to Rectangles Perimeter problem (https://www.spoj.com/problems/MMAXPER/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_usize_vec() -> Vec<u64> {
   let mut string = String::new();
   let _ = std::io::stdin().read_line(&mut string);
   return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn read_line_to_i64_vec() -> Vec<(u64, u64)> {
    let rectangle_cnt = read_line_to_usize();
    let mut sides = Vec::new();
    for _ in 0..rectangle_cnt {
        let rectangle = read_line_to_usize_vec();
        sides.push((rectangle[0], rectangle[1]));
    }
    return sides;
}

fn compute_maximum_perimeter(sides: &Vec<(u64, u64)>) -> u64 {

    let mut dp = vec![vec![0; 2]; sides.len()];
    dp[0][0] = sides[0].0;
    dp[0][1] = sides[0].1;

    for idx in 1..sides.len() {
        let difference1 = sides[idx - 1].1 as i64 - sides[idx].1 as i64;
        let difference2 = sides[idx - 1].0 as i64 - sides[idx].1 as i64;

        dp[idx][0] = std::cmp::max(
            dp[idx - 1][0] + difference1.abs() as u64 + sides[idx].0,
            dp[idx - 1][1] + difference2.abs() as u64 + sides[idx].0
        );

        let difference3 = sides[idx - 1].1 as i64 - sides[idx].0 as i64;
        let difference4 = sides[idx - 1].0 as i64 - sides[idx].0 as i64;
    
        dp[idx][1] = std::cmp::max(
            dp[idx - 1][0] + difference3.abs() as u64 + sides[idx].1,
            dp[idx - 1][1] + difference4.abs() as u64 + sides[idx].1
        );
    }

    return std::cmp::max(
        dp[sides.len() - 1][0],
        dp[sides.len() - 1][1]
    );
}

fn main() {
    let sides = read_line_to_i64_vec();
    let result = compute_maximum_perimeter(&sides);
    println!("{result}", result=result);
}