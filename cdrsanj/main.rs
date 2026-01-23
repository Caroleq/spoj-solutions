/// Solution to  CODER FIRST PROBLEM (https://www.spoj.com/problems/CDRSANJ/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn main() {
    let test_case = read_line_to_usize();

    if test_case <= 2 {
        println!("{result}", result=test_case);
        return;
    }

    let mut result = 0;
    let mut test_case = test_case;
    while test_case % 2 == 0 {
        result += 2;
        test_case /= 2;
    }
    
    println!("{result}", result=result);
}