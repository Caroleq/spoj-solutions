/// Solution to Candy III problem (https://www.spoj.com/problems/CANDY3/)

fn read_line_to_u64() -> u64 {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn main() {

    let cases_cnt = read_line_to_u64();
    for _ in 0..cases_cnt {
        let mut string = String::new();
        let _ = std::io::stdin().read_line(&mut string);
        let children_cnt = read_line_to_u64();
        let mut sum = 0;
        for _ in 0..children_cnt {
            let child_candies_cnt = read_line_to_u64();
            sum = (sum + child_candies_cnt) % children_cnt;
        }
        
        if sum == 0 {
            println!("YES");
        }
        else {
            println!("NO");
        }
    }
}