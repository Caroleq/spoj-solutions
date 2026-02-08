// Solution to Army Strength Problem (https://www.spoj.com/problems/ARMY/)

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

fn read_empty_line() {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
}


fn main() {
    let test_cases_cnt = read_line_to_usize();
    for test_idx in 0..test_cases_cnt {
        read_empty_line();
        let first_line = read_line_to_u64_vec();
        let godzilla_cnt = first_line[0];
        let mecha_godzilla_cnt = first_line[1];

        if mecha_godzilla_cnt == 0 && godzilla_cnt == 0 {
            read_empty_line();
            read_empty_line();
            println!("uncertain");
            continue;
        }

        if mecha_godzilla_cnt == 0 {
            read_empty_line();
            println!("Godzilla");
            continue;
        }

        if godzilla_cnt == 0 {
            read_empty_line();
            println!("MechaGodzilla");
            continue;
        }

        let godzilla = read_line_to_u64_vec();
        let mecha_godzilla = read_line_to_u64_vec();

        let max_element_godzilla = godzilla.into_iter().max().unwrap();
        let max_element_mecha_godzilla = mecha_godzilla.into_iter().max().unwrap();
        if max_element_mecha_godzilla > max_element_godzilla {
            println!("MechaGodzilla");
        } 
        else {
            println!("Godzilla");
        }        

    }
}