/// Solution to the "Triple Fat Ladies" problem on SPOJ (https://www.spoj.com/problems/EIGHTS/)

/*
The solution can be found by solving the equation:
(x + 10y + 100z) ** 3 mod 1000 == 888
where x, y, z are digits from 0 to 9 representing the last three digits of the number.
*/

fn read_line_to_u64() -> u64 {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse::<u64>().unwrap();
}

fn main() { 

    let cases_cnt = read_line_to_u64();
    for _ in 0..cases_cnt {
        let number = read_line_to_u64();
        let result = 192 + (number - 1)* 250;
        println!("{result}", result=result);
    }
}