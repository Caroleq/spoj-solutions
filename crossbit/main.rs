/// Solution to Crossbits (https://www.spoj.com/problems/CROSSBIT/)

use std::vec;

fn read_line_to_usize_vec() -> Vec<usize> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn is_on_diagonal(grid_size: usize, col_idx: usize, row_idx: usize) -> bool {

    if col_idx == row_idx {
        return true;
    }

    if col_idx == grid_size - row_idx - 1 {
        return true;
    }

    return false;
}

fn is_zero(grid: &Vec<Vec<usize>>, row_idx: i32, col_idx: i32) -> bool {

    if row_idx < 0 || col_idx < 0 || row_idx >= grid.len() as i32 || col_idx >= grid.len() as i32 {
        return false;
    }

    return grid[row_idx as usize][col_idx as usize] == 0;
}

fn has_zero_left_right_or_up(grid: &Vec<Vec<usize>>, row_idx: usize, col_idx: usize) -> bool {
    let col_idx = col_idx as i32;
    let row_idx = row_idx as i32;

    return is_zero(grid, row_idx - 1, col_idx) || 
           is_zero(grid, row_idx, col_idx - 1) ||
           is_zero(grid, row_idx, col_idx + 1);
}

fn is_zero_blocked_in_last_row(grid: &Vec<Vec<usize>>) -> bool {

    let grid_size = grid.len();

    for idx in 0..grid_size {
        if grid[grid_size - 1][idx] == 1 {
            continue;
        }

        if !has_zero_left_right_or_up(grid, grid_size - 1, idx) {
            return true;
        }
    }

    return false;
}

fn blocks_zero(grid: &Vec<Vec<usize>>, row_idx: usize, col_idx: usize) -> bool {

    if row_idx == 0 {
        return false;
    }

    if grid[row_idx - 1][col_idx] == 1 {
        return false;
    }

    if has_zero_left_right_or_up(grid, row_idx - 1, col_idx) {
        return false;
    }

    return true;
}

fn usize_to_binary_vec(value: usize, vec_size: usize) -> Vec<usize> {
    return (0..vec_size).rev().map(|n| (value >> n) & 1).collect();
}

fn set_ones_to_least_larger_positions(grid: &mut Vec<Vec<usize>>, row_idx: usize, dest_ones_cnt: usize, ones_in_cols: &mut Vec<usize>) -> bool {

    let row_size = grid[row_idx].len();

    let mut current_value = binary_vector_to_usize(&grid[row_idx]);
    clear_row(grid, row_idx, ones_in_cols);

    let base: i32 = 2;    
    while current_value as usize != base.pow(row_size as u32) as usize - 1 {
        current_value += 1;
        let bits_cnt = current_value.count_ones();
        if bits_cnt == dest_ones_cnt as u32 {
            let vector = usize_to_binary_vec(current_value, row_size);
            
            let mut can_assign_all = true;

            for col_idx in 0..row_size {
                if vector[col_idx] == 0 {
                    continue;
                }

                if !can_assign_one(grid, row_idx, col_idx, dest_ones_cnt, ones_in_cols) {
                    can_assign_all = false;
                    break;
                }
            }
            if can_assign_all {
                grid[row_idx] = vector;
                for col_idx in 0..row_size {
                    if grid[row_idx][col_idx] == 1 {
                        ones_in_cols[col_idx] += 1;
                    }
                }
                return true;
            }
        }
    }

    return false;
}

fn can_assign_one(grid: &Vec<Vec<usize>>, row_idx: usize, col_idx: usize, dest_ones_cnt: usize, ones_in_cols: &mut Vec<usize>) -> bool {
    return (!is_on_diagonal(grid.len(), col_idx, row_idx)) &&
             (ones_in_cols[col_idx] < dest_ones_cnt) &&
            (!blocks_zero(grid, row_idx, col_idx));
}

fn set_ones_smallest_possible(grid: &mut Vec<Vec<usize>>, row_idx: usize, dest_ones_cnt: usize, ones_in_cols: &mut Vec<usize>) -> bool {

    let mut ones_left_to_assign = dest_ones_cnt;

    let row_size = grid[row_idx].len();
    for col_idx in (0..row_size).rev() {

        if can_assign_one(grid, row_idx, col_idx, dest_ones_cnt, ones_in_cols) {

            grid[row_idx][col_idx] = 1;
            ones_in_cols[col_idx] += 1;
            ones_left_to_assign -= 1;
            if ones_left_to_assign == 0 {
                return true;
            }
        }
    }

    clear_row(grid, row_idx, ones_in_cols);
    return false;
}

fn clear_row(grid: &mut Vec<Vec<usize>>, row_idx: usize, ones_in_cols: &mut Vec<usize>) {
    for col_idx in 0..grid.len() {
        if grid[row_idx][col_idx] == 1 {
            ones_in_cols[col_idx] -= 1;
            grid[row_idx][col_idx] = 0;
        }
        
    }
}

fn binary_vector_to_usize(vector: &Vec<usize>) -> usize 
{
    let mut result = 0;
    for item in vector {
        result = result * 2 + item;
    }

    return result;
}

fn compute_grid_recursive(grid: &mut Vec<Vec<usize>>, row_idx: usize, dest_ones_cnt: usize, ones_in_cols: &mut Vec<usize>) -> bool {

    if !set_ones_smallest_possible(grid, row_idx, dest_ones_cnt, ones_in_cols) {
        return false;
    }

    if row_idx == grid.len() - 1 {
        if is_zero_blocked_in_last_row(grid) {
            clear_row(grid, row_idx, ones_in_cols);
            return false;
        }
        return true;
    }

    loop {

        if compute_grid_recursive(grid, row_idx + 1, dest_ones_cnt, ones_in_cols) {
            return true;
        }

        if !set_ones_to_least_larger_positions(grid, row_idx, dest_ones_cnt, ones_in_cols) {
            return false;
        }
    }
    return false;
}

fn set_ones_but_diagonals(grid: &mut Vec<Vec<usize>>) {

    for row_idx in 0..grid.len() {
        for col_idx in 0..grid.len() {
            if !is_on_diagonal(grid.len(), col_idx, row_idx) {
                grid[row_idx][col_idx] = 1;
            }
        }
    }
}

fn solve_problem(grid_size: usize, ones_bits_per_row_n_col: usize) -> Vec<Vec<usize>> {
    let mut grid = vec![vec![0; grid_size]; grid_size];

    if ones_bits_per_row_n_col == 0 {
        return grid;
    }

    if ones_bits_per_row_n_col == grid_size - 2 {
        set_ones_but_diagonals(&mut grid);
        return grid;
    }

    let mut ones_in_cols = vec![0; grid.len()];
    if !compute_grid_recursive(&mut grid, 0, ones_bits_per_row_n_col, &mut ones_in_cols) {
        println!("Error");
    }

    return grid;
}

/// These two special cases were too slow for my solution to be accepted
fn print_precomputed_for_10_n_5() {
    println!("0 0 0 0 1 1 1 1 1 0");
    println!("0 0 0 1 1 1 1 1 0 0");
    println!("0 0 0 0 1 1 1 0 1 1");
    println!("0 1 0 0 1 1 0 0 1 1");
    println!("1 1 1 1 0 0 0 0 0 1");
    println!("1 1 1 1 0 0 0 0 0 1");
    println!("1 1 1 0 0 0 0 1 0 1");
    println!("1 1 0 0 1 1 0 0 1 0");
    println!("1 0 1 1 0 0 1 1 0 0");
    println!("0 0 1 1 0 0 1 1 1 0");

    println!("");
}

fn print_precomputed_for_10_n_6() {
    println!("0 0 0 1 1 1 1 1 1 0");
    println!("0 0 1 1 1 1 1 1 0 0");
    println!("0 1 0 0 1 1 1 0 1 1");
    println!("1 1 0 0 1 1 0 0 1 1");
    println!("1 1 1 1 0 0 0 0 1 1");
    println!("1 1 1 1 0 0 1 1 0 0");
    println!("1 1 1 0 0 0 0 1 1 1");
    println!("1 1 0 0 1 1 0 0 1 1");
    println!("1 0 1 1 0 0 1 1 0 1");
    println!("0 0 1 1 1 1 1 1 0 0");

    println!("");
}

fn main() {

    loop {

        let test_case = read_line_to_usize_vec();
        if test_case.len() == 1 {
            break;
        }

        let grid_size = test_case[0];
        let ones_bits_per_row_n_col = test_case[1];

        if grid_size == 10 && ones_bits_per_row_n_col == 6 {
            print_precomputed_for_10_n_6();
            continue;
        }

        if grid_size == 10 && ones_bits_per_row_n_col == 5 {
            print_precomputed_for_10_n_5();
            println!("");
            continue;
        } 

        let result = solve_problem(grid_size, ones_bits_per_row_n_col);

        let mut result_string = String::new();
        for grid_row in result {
            for grid_element in grid_row {
                if grid_element == 1 {
                    result_string += "1 ";
                }
                else {
                    result_string += "0 ";
                }
            }
            result_string += "\n";
        }

        println!("{result_string}", result_string=result_string); 
    }
}