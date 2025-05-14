/// Solution to Bracket Check problem (https://www.spoj.com/problems/BRCKTCHK/)

fn read_line_to_u8_vec() -> Vec<u8> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().as_bytes().to_vec();
}

fn brackets_correct(brackets: &Vec<u8>) -> bool {
    let mut bracket_stack = Vec::new();

    for bracket in brackets.iter() {
        match bracket {
            40 | 123 | 91 | 60 => bracket_stack.push(*bracket),
            41 | 125 | 93 | 62 => {

                if bracket_stack.is_empty() {
                    return false;
                }

                let top_bracket = bracket_stack.pop().unwrap();
                if top_bracket == 40 && *bracket == 41 ||
                    top_bracket == 123 && *bracket == 125 ||
                    top_bracket == 91 && *bracket == 93 ||
                    top_bracket == 60 && *bracket == 62 {
                    
                    continue;
                }
                return false;
            },
            _ => panic!("Incorrect bracket value")
        };
    }

    return bracket_stack.is_empty();
}


fn main() {

    let brackets = read_line_to_u8_vec();
    if brackets_correct(&brackets) {
        println!("Yes");
    }
    else {
        println!("No");
    }
}