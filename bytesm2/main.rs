/// Solution to BYTESM2 Problem (https://www.spoj.com/problems/BYTESM2/)

/// Note: BYTESM2 Problem **does not** accept Rust solutions for the moment. I noticed it when I wanted to submit this solution.
/// Nonetheless I decided to post this code. 

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
 }

fn read_line_to_usize_vec() -> Vec<u8> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
 }

fn parse_board_from_input() -> Vec<Vec<u8>> {
    let dimensions = read_line_to_usize_vec();
    let h = dimensions[0];

    let mut board: Vec<Vec<u8>> = Vec::new();
    for _ in 0..h {
        let row = read_line_to_usize_vec();
        board.push(row);
    }
    
    return board;
}

fn compute_max_stone_count(board: &Vec<Vec<u8>>) -> u32 {
    let h = board.len();
    let w = board[0].len();

    let mut dp: Vec<Vec<u32>> = vec![vec![0; w]; h];

    for j in 0..w {
        dp[0][j] = board[0][j] as u32; 
    }

    for i in 1..h {
        for j in 0..w {

            dp[i][j] = dp[i - 1][j];
            if j > 0 {
                dp[i][j] = std::cmp::max(dp[i][j], dp[i - 1][j - 1]);
            }
            if j < w - 1 {
                dp[i][j] = std::cmp::max(dp[i][j], dp[i - 1][j + 1]);
            }

            dp[i][j] += board[i][j] as u32;
        }
    }

    return *dp[h - 1].iter().max().unwrap();
}


fn main() {
    let case_number = read_line_to_usize();

    for _ in 0..case_number {
        let board = parse_board_from_input();

        let max_stone_count = compute_max_stone_count(&board);
        println!("{max_stone_count}", max_stone_count=max_stone_count);
    }
    
}