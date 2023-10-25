/// Solution to SCYPHER (Substitution Cipher) problem (https://www.spoj.com/problems/SCYPHER/)

use std::collections::HashSet;

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_usize_vec() -> Vec<usize> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
 }

fn read_strings(strings_cnt: usize) -> Vec<Vec<u8>> {
    let mut strings = Vec::new();
    for _ in 0..strings_cnt {
        let mut string = String::new();
        let _ = std::io::stdin().read_line(&mut string);
        strings.push(string.trim().as_bytes().to_vec());
    }
    return strings;
}

static A_ASCII_CODE: u8 = 97;

fn create_dependency_graph(encrypted_strings: &Vec<Vec<u8>>, alphabet_size: usize) -> (Vec<HashSet<usize>>, Vec<usize>) {
   let mut graph = vec![HashSet::new(); alphabet_size];
   let mut counts = vec![0; alphabet_size];

   for str_idx in 0..encrypted_strings.len() - 1 {

     let str1 = &encrypted_strings[str_idx];
     let str2 = &encrypted_strings[str_idx + 1];

     for idx in 0..std::cmp::min(str1.len(), str2.len()) {
        if str1[idx] != str2[idx] {

            let str1_idx = (str1[idx] - A_ASCII_CODE) as usize;
            let str2_idx = (str2[idx] - A_ASCII_CODE) as usize;

            if !graph[str1_idx].contains(&str2_idx) {
                graph[str1_idx].insert(str2_idx);
                counts[str2_idx] += 1;
            }
            break;
        }
     }
   }
   return (graph, counts);
}

fn decrypt_message(substitution_table: &Vec<u8>, encrypted_message: &Vec<u8>) -> Vec<u8> {
    let mut decrypted_message = Vec::new();

    for character in encrypted_message {
        if *character < A_ASCII_CODE || (*character - A_ASCII_CODE) as usize >= substitution_table.len() {
            decrypted_message.push(*character);
        }
        else {
            decrypted_message.push( substitution_table[(*character - A_ASCII_CODE) as usize] + A_ASCII_CODE );
        }
    }
    return decrypted_message;
} 

fn compute_substitution_table(graph: &Vec<HashSet<usize>>, counts: &mut Vec<usize>, alphabet_size: usize) -> (bool, Vec<u8>) {
    let mut table = vec![0; alphabet_size];

    let mut letter_idx = -1;
    for idx in 0..counts.len() {
        if counts[idx] == 0 {
            if letter_idx != -1 {
                return (false, table);
            }
            letter_idx = idx as i32;
        }
    }

    if letter_idx == -1 {
        return (false, table);
    }

    let mut ordering_number: usize = 0;
    loop {
        table[letter_idx as usize] = ordering_number as u8;
        ordering_number += 1;

        if ordering_number == alphabet_size {
            return (true, table);
        }

        let mut next_idx = -1;
        for neighbor in graph[letter_idx as usize].iter() {
            counts[*neighbor] -= 1;
            if counts[*neighbor] == 0 {
                    if next_idx != -1 {
                        return (false, table);
                    }
                    next_idx = *neighbor as i32;
                }
            }
        if next_idx == -1 {
            return (false, table);
        }
        letter_idx = next_idx as i32; 
    } 
}

fn main() {
    let cases_count = read_line_to_usize();
    
    for _ in 0..cases_count {
        let first_line = read_line_to_usize_vec();
        let alphabet_size = first_line[0];
        let encrypted_cases_count = first_line[1];

        let encrypted_strings = read_strings(encrypted_cases_count);
        let mut secret_message = String::new();
        let _ = std::io::stdin().read_line(&mut secret_message);
        let secret_message_bytes = secret_message.trim().as_bytes().to_vec();

        let (graph, mut counts) = create_dependency_graph(&encrypted_strings, alphabet_size);

        let (result, table) = compute_substitution_table(&graph, &mut counts, alphabet_size);
        if !result {
            println!("Message cannot be decrypted.");
        }
        else {
            let decrypted_message = decrypt_message(&table, &secret_message_bytes);

            let decrypted_message_str = match std::str::from_utf8(&decrypted_message) {
                   Ok(v) => v,
                   Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
             };

             println!("{decrypted_message_str}", decrypted_message_str=decrypted_message_str);
        }
    }
}