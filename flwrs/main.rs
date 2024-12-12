/// Solution to Flowers problem (https://www.spoj.com/problems/FLWRS/)

/// Solved using:
/// https://math.stackexchange.com/questions/2010215/how-many-permutations-with-no-adjacent-characters
/// https://oeis.org/A002464

fn read_line_to_u64() -> u64 {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn compute(pots_cnt: u64, modulo: u64) -> u64 {
    let pots_cnt = pots_cnt as i64;
    let modulo = modulo as i64;

    let mut counts = vec![1, 1, 0, 0];
    let counts_len = counts.len();

    // a(n) = (n+1)*a(n-1) - (n-2)*a(n-2) - (n-5)*a(n-3) + (n-3)*a(n-4)
    for pot_idx in 4..pots_cnt + 1 {
        counts[pot_idx as usize % counts_len] = (
                    ((pot_idx + 1) * counts[(pot_idx as usize - 1) % counts_len] % modulo) - 
                    ((pot_idx - 2) * counts[(pot_idx as usize - 2) % counts_len] % modulo) - 
                    ((pot_idx - 5) * counts[(pot_idx as usize - 3) % counts_len] % modulo) + 
                    ((pot_idx - 3) * counts[(pot_idx as usize - 4) % counts_len] % modulo)
                 ) % modulo;
        while counts[pot_idx as usize % counts_len] < 0 {
            counts[pot_idx as usize % counts_len] += modulo;
        }
    }

    return counts[pots_cnt as usize % counts.len()] as u64;   
}

fn main() {
    let pots_cnt = read_line_to_u64();
    let modulo = read_line_to_u64();

    let result = compute(pots_cnt, modulo);
    println!("{result}", result=result); 
}