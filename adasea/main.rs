/// Solution to Ada and Sea Problem (https://www.spoj.com/problems/ADASEA/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_lines_to_u8_vecs(lines_number: usize) -> Vec<Vec<u8>> {
    let mut vector = Vec::new();

    for _ in 0..lines_number {
        let mut string = String::new();
        let _ = std::io::stdin().read_line(&mut string);

        vector.push(string.trim().bytes().collect());
    }

    return vector;
}

fn read_line_to_usize_vec() -> Vec<usize> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn traverse_island(grid: &Vec<Vec<u8>>, visited: &mut Vec<Vec<bool>>, left_idx: usize, right_idx: usize) -> u64 {

    let mut result = 1;
    visited[left_idx][right_idx] = true;

    let island_char = 35;

    if left_idx > 0 {
        if !visited[left_idx - 1][right_idx] && grid[left_idx - 1][right_idx] == island_char {
            result += traverse_island(grid, visited, left_idx - 1, right_idx);
        } 
    }

    if left_idx < grid.len() - 1 {
        if !visited[left_idx + 1][right_idx] && grid[left_idx + 1][right_idx] == island_char {
            result += traverse_island(grid, visited, left_idx + 1, right_idx);
        } 
    }

    if right_idx > 0 {
        if !visited[left_idx][right_idx - 1] && grid[left_idx][right_idx - 1] == island_char {
            result += traverse_island(grid, visited, left_idx, right_idx - 1);
        } 
    }

    if right_idx < grid[0].len() - 1 {
        if !visited[left_idx][right_idx + 1] && grid[left_idx][right_idx + 1] == island_char {
            result += traverse_island(grid, visited, left_idx, right_idx + 1);
        } 
    }   

    return result;
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    while m != 0 {
        if m < n {
          std::mem::swap(&mut m, &mut n);
        }
        m %= n;
      }
      return n;
}

fn compute_expected_island_size(grid: &Vec<Vec<u8>>) -> (u64, u64) {

    let island_char = 35;

    let mut island_sizes = Vec::new();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    for i_idx in 0..grid.len() {
        for j_idx in 0..grid[0].len() {

            
            if grid[i_idx][j_idx] == island_char && !visited[i_idx][j_idx] {
                let island_size = traverse_island(&grid, &mut visited, i_idx, j_idx); 
                island_sizes.push(island_size);
            }
        }
    }

    if island_sizes.is_empty() {
        return (0, 1);
    }

    let mut numerator = 0;
    for size in island_sizes {
        numerator += size * size;
    }

    let denominator = (grid[0].len() * grid.len()) as u64;

    let divide_factor = gcd(numerator, denominator);
    return (numerator / divide_factor, denominator / divide_factor);
}

fn main() {
    let test_cases_cnt = read_line_to_usize();

    for _ in 0..test_cases_cnt {
        let first_line_in_case = read_line_to_usize_vec();
        let rows_cnt = first_line_in_case[0];
        let grid = read_lines_to_u8_vecs(rows_cnt);

        let result = compute_expected_island_size(&grid);
        if result.1 == 1 {
            println!("{p}", p=result.0);
        }
        else {
            println!("{p} / {q}", p=result.0, q=result.1);
        }
    }
}