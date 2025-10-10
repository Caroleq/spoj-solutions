/// Solution to Transform the Expression problem (https://www.spoj.com/problems/ONP/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_u8_vec() -> Vec<u8> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.as_bytes().to_vec();
}

fn is_symbol(char_value: u8) -> bool {
    if char_value >= 97 && char_value <= 122 {
        return true;
    }
    return false;
}

fn is_right_parenthesis(char_value: u8) -> bool {
    return char_value == 41;
}

fn is_operation(char_value: u8) -> bool {
    if  char_value == 42 || 
        char_value == 43 || 
        char_value == 45 || 
        char_value == 47 ||
        char_value == 94 {
        return true;    
    }
    return false;
}

fn solve_problem(expr: &Vec<u8>) -> Vec<u8> {
    let mut stack = Vec::new();
    let mut result = Vec::new();
    let mut idx = 0;
    while idx < expr.len() {
        let s = expr[idx];
        if is_operation(s)
        {
            stack.push(s);
        }
        else if is_symbol(s) {
            result.push(s);
        }
        else if is_right_parenthesis(s) {
            result.push(*stack.last().unwrap());
            stack.pop();
        }
        idx += 1;
    }
    return result;
}

fn main() {
    let test_case_cnt = read_line_to_usize();

    for _ in 0..test_case_cnt {
        let expr = read_line_to_u8_vec();
        let result = solve_problem(&expr);
        println!("{result}", result=String::from_utf8_lossy(&result));
    }   
}