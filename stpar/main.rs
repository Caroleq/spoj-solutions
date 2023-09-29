///  Solution to STPAR Problem (https://www.spoj.com/problems/STPAR/)


fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an integer");
 }
 
fn read_line_to_usize_vec() -> Vec<usize> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn check_reordering(mobiles: &Vec<usize>) -> bool {

    let mut help_stack = Vec::new();
    let mut current_expected = 1;

    for mobile in mobiles {

        if *mobile == current_expected {
            current_expected += 1;
            while !help_stack.is_empty() {
                if *help_stack.last().expect("unexpected") == current_expected {
                    current_expected += 1;
                    help_stack.pop();
                }
                else {
                    break;
                }

            }
        }
        else {
            help_stack.push(*mobile);
        }
    }

    while !help_stack.is_empty() {
        if *help_stack.last().expect("unexpected") == current_expected {
            current_expected += 1;
            help_stack.pop();
        }
        else {
            return false;
        }

    }
    return true;
}

fn main() {
    let mut n = read_line_to_usize();
    while n != 0 {
        let mobiles = read_line_to_usize_vec();

        if check_reordering(&mobiles) {
            println!("yes");
        }
        else {
            println!("no");
        }

        n = read_line_to_usize();
    }
}