/// Solution to Primal Fear (https://www.spoj.com/problems/VECTAR8/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_lines_to_usize_vec(lines_cnt: usize) -> Vec<usize> {
    let mut vector = Vec::new();
    for _ in 0..lines_cnt {
        vector.push(read_line_to_usize() as usize);
    }
    return vector;
}

fn truncate_most_significant_digit(number: usize) -> usize
{
    if number < 100 {
        return number % 10;
    }
    else if number < 1000 {
        return number % 100;
    }
    else if number < 10000 {
        return number % 1000;
    }
    else if number < 100000 {
        return number % 10000;
    }   
    else if number < 1000000 {
        return number % 100000;
    }
    else {
        return number % 1000000;
    }
}

fn get_digits_count(number: usize) -> usize
{
    if number < 10 {
        return 1;
    }
    else if number < 100 {
        return 2;
    }
    else if number < 1000 {
        return 3;
    }
    else if number < 10000 {
        return 4;
    }
    else if number < 100000 {
        return 5;
    }   
    else if number < 1000000 {
        return 6;
    }
    else {
        return 7;
    }
}

fn count_feared_primes(number: usize) -> Vec<usize> {

    let mut prime_flags = vec![0; number as usize + 1];
    let mut counts = vec![0; number as usize + 1];
    prime_flags[0] = 1;
    prime_flags[1] = 1;
    let number_sqrt = (number as f64).sqrt() as usize + 1;
    for value in 2..number_sqrt {
        if prime_flags[value] == 0 {
            if value < 10 {
                counts[value] = counts[value - 1] + 1;
                prime_flags[value] = 2;
            }
            else {
                let truncated_number = truncate_most_significant_digit(value);
                if prime_flags[truncated_number] == 2 && 
                    get_digits_count(value) == get_digits_count(truncated_number) + 1 {
                    prime_flags[value] = 2;
                    counts[value] = counts[value - 1] + 1;
                }
                else {
                    counts[value] = counts[value - 1];
                }               
            }
        }
        else {
            counts[value] = counts[value - 1];
            continue;
        }

        let mut value_multiples = value * value;
        while value_multiples < number + 1 {
            prime_flags[value_multiples] = 1;
            value_multiples += value;
        }
    }

    for value in number_sqrt..number + 1 {
        if prime_flags[value] == 0 {
            if get_digits_count(value) == 1 {
                prime_flags[value] = 2;
                counts[value] = counts[value - 1] + 1;
                continue;
            }
            let truncated_number = truncate_most_significant_digit(value);
            if prime_flags[truncated_number] == 2 && 
                get_digits_count(value) == get_digits_count(truncated_number) + 1{
                prime_flags[value] = 2;
                counts[value] = counts[value - 1] + 1;
            }
            else {
                counts[value] = counts[value - 1];
            }  
        }
        else {
            counts[value] = counts[value - 1];
        }
    }
    return counts;
}

fn main() {
    let tests_cnt = read_line_to_usize();
    if tests_cnt == 0 {
        return;
    }

    let test_cases = read_lines_to_usize_vec(tests_cnt);
    let results = count_feared_primes(*test_cases.iter().max().unwrap());
    for case in test_cases {
        println!("{result}", result=results[case]);
    }
}