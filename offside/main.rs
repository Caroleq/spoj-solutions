/// Solution to He is offside (https://www.spoj.com/problems/OFFSIDE/) Problem.


fn read_line_to_usize_vec() -> Vec<usize> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn main() {
    loop {
        let test_case = read_line_to_usize_vec();
        let attackers_cnt = test_case[0];
        let defenders_cnt = test_case[1];
        
        if attackers_cnt == 0 && defenders_cnt == 0 {
            break;
        }

        let mut attackers_distances = read_line_to_usize_vec();
        let mut defenders_distances = read_line_to_usize_vec();

        attackers_distances.sort();
        defenders_distances.sort();

        if attackers_distances[0] < defenders_distances[1] {
            println!("Y");
        } 
        else {
            println!("N");
        }
    }
}