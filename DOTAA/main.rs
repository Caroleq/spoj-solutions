/// Solution to DOTA HEROES (https://www.spoj.com/problems/DOTAA/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_usize_vec() -> Vec<usize> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn main() {
    let test_case_cnt = read_line_to_usize();

    for _ in 0..test_case_cnt {
        let first_line_of_test_case = read_line_to_usize_vec();
        let heroes_cnt = first_line_of_test_case[0];
        let towers_cnt = first_line_of_test_case[1];
        let damage = first_line_of_test_case[2];

        let mut maximum_towers = 0;
        for _ in 0..heroes_cnt {
            let hero_health = read_line_to_usize();
            maximum_towers += (hero_health - 1) / damage;
        }

        if maximum_towers >= towers_cnt {
            println!("YES");
        }
        else {
            println!("NO");
        }
    }
}