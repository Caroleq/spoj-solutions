/// Solution to Julka (https://www.spoj.com/problems/JULKA/)

fn read_line_to_digits_vec() -> Vec<u8> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    let mut digits = string.trim().as_bytes().to_vec();
    let zero_ascii = 48;
    for idx in 0..digits.len() {
        digits[idx] -= zero_ascii;
    }
    return digits;
}

fn subtract(minuend: &Vec<u8>, subtrahend: &Vec<u8>) -> Vec<u8> {
    let mut minuend = minuend.to_vec();
    minuend.reverse();
    let mut subtrahend = subtrahend.to_vec();
    subtrahend.reverse();

    let mut subtracted = Vec::new();

    let mut credit = 0;
    for idx in 0..subtrahend.len() {
        let mut result = 0;
        if minuend[idx] >= subtrahend[idx] {
            result = minuend[idx] - subtrahend[idx];
            if credit > 0 {
                if result > 0 {
                    result -= 1;
                    credit = 0;
                }
                else {
                    result += 9;
                }
            }
        }
        else {
            result = 10 - credit + minuend[idx] - subtrahend[idx];
            credit = 1;
        }
        subtracted.push(result);
    }

    for idx in subtrahend.len()..minuend.len() {
        if credit == 1 {
            if minuend[idx] > 0 {
                subtracted.push(minuend[idx] - 1);
                credit = 0;
            }
            else {
                subtracted.push(9);
            }
        }
        else {
            subtracted.push(minuend[idx]);
        }
    }

    while subtracted.len() > 1 && *subtracted.last().unwrap() == 0 {
        subtracted.pop();
    }

    subtracted.reverse();

    if subtracted.is_empty() {
        subtracted.push(0);
    }

    return subtracted;
}

fn divide_by_2(number: &Vec<u8>) -> Vec<u8> {
   let mut divided = Vec::new();

   let mut accumulator = 0;
   for idx in 0..number.len() {
    let result = (accumulator * 10 + number[idx]) / 2;
    accumulator =  accumulator * 10 + number[idx] - result * 2;
    if divided.is_empty() && result == 0 {
        continue;
    }
    divided.push(result);
   }

   if divided.is_empty() {
    divided.push(0);
   }

   return divided;
}

fn vec_to_ascii_repr(number_vec: &Vec<u8>) -> Vec<u8> {
    let mut ascii_repr = Vec::new();
    let zero_ascii = 48;
    for number in number_vec {
        ascii_repr.push(number + zero_ascii);
    }
    return ascii_repr;
}

fn main() {
    let cases_cnt = 10;

    for _ in 0..cases_cnt {

        let apples_sum = read_line_to_digits_vec();
        let difference = read_line_to_digits_vec();

        let natalia_apples_cnt = divide_by_2(
            &subtract(&apples_sum, &difference)
        );

        let natalia_apples_cnt_ascii_repr = vec_to_ascii_repr(&natalia_apples_cnt);
        let natalia_apples_string = std::str::from_utf8(&natalia_apples_cnt_ascii_repr).unwrap();
        
        let klaudia_apples_cnt_ascii_repr = vec_to_ascii_repr(&subtract(&apples_sum, &natalia_apples_cnt));
        let klaudia_apples_string = std::str::from_utf8(&klaudia_apples_cnt_ascii_repr).unwrap();
        
        println!("{klaudia_apples}", klaudia_apples=klaudia_apples_string);
        println!("{natalia_apples}", natalia_apples=natalia_apples_string);
    }
}

