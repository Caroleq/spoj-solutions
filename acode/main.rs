///  Solution to ACODE Problem (https://www.spoj.com/problems/ACODE/)

fn count_decodings(code: &str) -> u64 {

    if code.len() == 1 {
        return 1;
    }

    let mut decoding_counts: Vec<u64> = vec![0; code.len()];
    decoding_counts[0] = 1;

    let n = code[0..2].parse::<u32>().expect("unexpected");
    if n < 27 && n != 10 && n != 20 {
        decoding_counts[1] = 2;
    }
    else {
        decoding_counts[1] = 1;
    }

    let mut idx = 2;
    while idx < code.len() {
        let character = code.chars().nth(idx).expect("unexpected");
        if character  == '0' {
            decoding_counts[idx] = decoding_counts[idx - 2];
        }
        else {
            decoding_counts[idx] = decoding_counts[idx - 1];

            let n = code[(idx - 1)..(idx + 1)].parse::<u32>().expect("unexpected");
            if n < 27 && n > 10 {
                decoding_counts[idx] += decoding_counts[idx - 2];
            }
        }
        idx += 1;
    }
    
    return *decoding_counts.last().expect("unexpected");
}


fn main() {

    loop {
        let mut line = String::new();
        let _ = std::io::stdin().read_line(&mut line);

        if line.chars().nth(0).expect("unexpected") == '0' {
            break;
        }

        let code = line.trim();

        let decodings_count = count_decodings(&code);
        println!("{decodings_count}", decodings_count=decodings_count);
    }

}