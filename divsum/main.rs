/// Solution to Divisor Summation (https://www.spoj.com/problems/DIVSUM/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an usize integer");
}

fn generate_divisor_sums(max_number: u64) -> Vec<u64> {

    let mut div_sums = vec![1; max_number as usize + 1];
    div_sums[0] = 0;
    div_sums[1] = 0;
    let mut tmp_sums = vec![0; max_number as usize + 1];
    for number in 2..max_number + 1 {
        if div_sums[number as usize] > 1 {
            continue;
        }

        let mut prime_exp = number;
        while prime_exp < max_number + 1 {
            let mut prime_exp_multiples = prime_exp;
            while prime_exp_multiples < max_number + 1 {
                tmp_sums[prime_exp_multiples as usize] += prime_exp;
                prime_exp_multiples += prime_exp;
            }
            prime_exp *= number;
        }

        let mut prime_multiples = number;
        while prime_multiples < max_number + 1 {
            div_sums[prime_multiples as usize] += tmp_sums[prime_multiples as usize] * div_sums[prime_multiples as usize];
            tmp_sums[prime_multiples as usize] = 0;
            prime_multiples += number;
        }   
    }

    for number in 2..max_number + 1 {
        div_sums[number as usize] -= number;
    }

    return div_sums;
}

fn main() {
    let div_sums = generate_divisor_sums(500000);

    let cases_cnt = read_line_to_usize();
    for _ in 0..cases_cnt {
        let number = read_line_to_usize();
        println!("{sum}", sum=div_sums[number]);
    }
}