/// Solution to Girls and Boys problem (https://www.spoj.com/problems/GIRLSNBS/)

fn read_line_to_i64_vec() -> Vec<i64> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn compute(girls_cnt: u64, boys_cnt: u64) -> u64 {
    if girls_cnt > boys_cnt {
        return (girls_cnt + boys_cnt) / (boys_cnt + 1);
    }
    return (girls_cnt + boys_cnt) / (girls_cnt + 1);
}

fn main() {

    loop {
        let case = read_line_to_i64_vec();
        let girls_cnt = case[0];
        let boys_cnt = case[1];

        if girls_cnt == -1 {
            break;
        }

        let result = compute(girls_cnt as u64, boys_cnt as u64);
        println!("{result}", result=result);
    }
}