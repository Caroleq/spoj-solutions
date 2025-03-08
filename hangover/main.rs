


fn read_line_to_f32() -> f32 {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}


fn compute_cards_count(overhang: f32) -> usize {

    let mut sum = 0.0;
    let mut cards_cnt = 1;
    while sum < overhang {
        sum += 1.0 / (cards_cnt as f32 + 1.0);
        cards_cnt += 1;
    }

    return cards_cnt - 1;
}

fn main() {
    loop {
        let overhang = read_line_to_f32();
        if overhang < 0.01 {
            break;
        }

        let cards_cnt = compute_cards_count(overhang);
        println!("{cards_cnt} card(s)", cards_cnt=cards_cnt);
    }
}