/// Solution to To and Fro Problem (https://www.spoj.com/problems/TOANDFRO/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_encoded_message() -> Vec<u8> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    if string.ends_with('\n') {
        string.pop();
        if string.ends_with('\r') {
            string.pop();
        }
    }
    return string.as_bytes().to_vec();
}

fn decode_message(encoded_message: &Vec<u8>, columns_cnt: usize) -> String {
    let mut plaintext_message = vec![0; encoded_message.len()];

    let mut encoded_msg_idx = 0;
    let mut left_to_right = true;
    let mut first_row_idx = 0;

    for plain_msg_idx in 0..plaintext_message.len() {
        plaintext_message[plain_msg_idx] = encoded_message[encoded_msg_idx];

        if left_to_right {
            encoded_msg_idx += (2 * columns_cnt) - 1 - 2 * first_row_idx;
            left_to_right = false;
        }
        else {
            encoded_msg_idx += 1 + 2 * first_row_idx;
            left_to_right = true;
        }
        if encoded_msg_idx >= plaintext_message.len() {
            encoded_msg_idx = first_row_idx + 1;
            first_row_idx += 1;
            left_to_right = true;
        }
    }

    return String::from_utf8(plaintext_message).unwrap();
}

fn main() {
    loop {
        let columns_cnt = read_line_to_usize();
        if columns_cnt == 0 {
            break;
        }
        let encoded_message = read_encoded_message();

        let plaintext_message = decode_message(&encoded_message, columns_cnt);
        println!("{plaintext_message}", plaintext_message=plaintext_message);
    }
}