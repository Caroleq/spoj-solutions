// Solution to Onotole needs your help (https://www.spoj.com/problems/OLOLO/)

use std::collections::HashSet;

fn read_line_to_u64() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}


fn main() {
    let pyanis_cnt = read_line_to_u64();

    let mut pyanis = HashSet::new();

    for _ in 0..pyanis_cnt {
        let pyani_number = read_line_to_u64();

        if pyanis.contains(&pyani_number) {
            pyanis.remove(&pyani_number);
        }
        else {
            pyanis.insert(pyani_number);
        }        

    }

    for pyani in &pyanis {
        println!("{pyani}", pyani=*pyani);
    }
}
