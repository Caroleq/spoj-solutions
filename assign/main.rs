// Solution to Assignments Problem (https://www.spoj.com/problems/ASSIGN/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_u64_vec() -> Vec<u64> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn read_preferences(students_cnt: usize) -> Vec<Vec<u64>> {
    let mut preferences = Vec::new();
    for _ in 0..students_cnt {
        preferences.push(read_line_to_u64_vec());
    }
    return preferences;
}

fn compute_assignments(dp: &mut Vec<u64>, preferences: &Vec<Vec<u64>>, mask: usize, students_cnt: usize, computed: &mut Vec<bool>) -> u64 {

    if students_cnt == 0 {
        computed[mask] = true;
        dp[mask] = preferences[students_cnt][mask.trailing_zeros() as usize];
        return dp[mask];
    }

    let mut assignment_cnt = 0;
    for bit_idx in 0..preferences.len() {
        if mask & (1 << bit_idx) > 0 && preferences[students_cnt][bit_idx] == 1 {
            
            let new_mask = mask & (!(1 << bit_idx));
            if computed[new_mask] {
                assignment_cnt += dp[new_mask];
            }
            else {
                assignment_cnt += compute_assignments(dp, preferences, new_mask, students_cnt - 1, computed);
            } 
        }
    }

    computed[mask] = true;
    dp[mask] = assignment_cnt;

    return assignment_cnt;
}

fn compute_assignment_cnt(preferences: &Vec<Vec<u64>>) -> u64 {
    let base: usize = 2;
    let mut dp = vec![0; base.pow(preferences.len() as u32)];
    let mut visited = vec![false; base.pow(preferences.len() as u32)];

    return compute_assignments(&mut dp, preferences, base.pow(preferences.len() as u32) - 1, preferences.len() - 1, &mut visited); 
}

fn main() {
    let test_cases_cnt = read_line_to_usize();

    for _ in 0..test_cases_cnt {
        let students_cnt = read_line_to_usize();
        let preferences = read_preferences(students_cnt);

        let result = compute_assignment_cnt(&preferences);
        println!("{result}", result=result);
    } 
}