/// Add solution to Anti-Blot System (https://www.spoj.com/problems/ABSYS/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_equation() -> String {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    string.clear();
    let _ = std::io::stdin().read_line(&mut string);
    return string;
}

fn compute_equation(equation_string: &String) -> (String, String, String) {
    let equality_split_eq = equation_string.trim().split(" = ").collect::<Vec<&str>>();
    let plus_split_eq = equality_split_eq[0].split(" + ").collect::<Vec<&str>>();

    let number1: Result<i64, _> = plus_split_eq[0].parse();
    let number2: Result<i64, _> = plus_split_eq[1].parse();
    let result: Result<i64, _> = equality_split_eq[1].parse();

    if plus_split_eq[0].contains("machula") {
        let return_value_int: i64 = result.expect("Error") - number2.expect("Error");  
        return (return_value_int.to_string(), plus_split_eq[1].to_string(), equality_split_eq[1].to_string());      
    }
    else if plus_split_eq[1].contains("machula") {
        let return_value_int: i64 = result.expect("Error") - number1.expect("Error");        
        return (plus_split_eq[0].to_string(), return_value_int.to_string(), equality_split_eq[1].to_string());      
    }
    else {
        let return_value_int: i64 = number1.expect("Error") + number2.expect("Error");        
        return (plus_split_eq[0].to_string(), plus_split_eq[1].to_string(), return_value_int.to_string());      
    }    
}

fn main() {

    let test_cases_cnt = read_line_to_usize();
    for _ in 0..test_cases_cnt {

        let equation_string = read_equation();
        let (added_number1, added_number2, sum) = compute_equation(&equation_string);

        println!("{added_number1} + {added_number2} = {sum}", added_number1=added_number1, added_number2=added_number2, sum=sum);
    }
}