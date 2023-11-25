/// Solution to BitPlay69 Problem (https://www.spoj.com/problems/BT69/)

/* Testing script usage:
$ python test_generator.py
$ ./main.exe < small.in > mine.out
$ diff -w mine.out small.out
*/


struct BitPlay {
    number_m_bits: Vec<u8>,
    number_n_bits: Vec<u8>,
}

fn bit_size(number: u64) -> usize {
    let mut bit_size_value = 0;
    let mut number = number;
    while number > 0 {
        number = number >> 1;
        bit_size_value += 1;
    }
    return bit_size_value;
}

fn u64_to_bit_vector(number: u64, vector_size: usize) -> Vec<u8> {
    let mut vector = vec![0; vector_size];
    let mut number = number;
    let mut position = vector.len();
    while number > 0 {
        position -= 1;
        if (number % 2) == 1 {
            vector[position] = 1;
        }
        number = number >> 1;
        if position == 0 && number > 0 {
            panic!("Vector size is too small for the number"); 
        }
    }
    return vector;
}

fn bit_vec_to_u64(vector: &Vec<u8>) -> u64 {
    let mut result: u64 = 0;
    let mut position = 0;

    while position < vector.len() {
        result += vector[position] as u64;
        result <<= 1;
        position += 1;
    }

    return result >> 1;
}

impl BitPlay {
    pub fn new(number_m: u64, number_n: u64) -> Self {
        let vector_sizes = bit_size(std::cmp::max(number_m, number_n));

        Self {
            number_m_bits: u64_to_bit_vector(number_m, vector_sizes), 
            number_n_bits: u64_to_bit_vector(number_n, vector_sizes),
        }
    }

    pub fn compute(&self) -> u64 {
        let mut n_bits_xor_k_bits = self.number_n_bits.to_vec();
        let status = self.compute_inner(0, &mut n_bits_xor_k_bits);
        if !status {
            let base: u64 = 2;
            return base.pow(self.number_m_bits.len() as u32);
        }

        let k_bits = n_bits_xor_k_bits.iter().zip(self.number_n_bits.iter()).map(|(&x1, &x2)| x1 ^ x2).collect();
        return bit_vec_to_u64(&k_bits);
    }

    fn compute_inner(&self, position: usize, n_bits_xor_k_bits: &mut Vec<u8>) -> bool {
        let original_pos = position;
        let position = self.find_differing_pos(n_bits_xor_k_bits, position);
        if position == usize::MAX {
            return self.set_smallest_result_bit(n_bits_xor_k_bits, self.number_m_bits.len() - 1, original_pos);
        }

        if self.number_n_bits[position] > self.number_m_bits[position] {
            return true;
        }
        n_bits_xor_k_bits[position] = 1;
        if self.compute_inner(position, n_bits_xor_k_bits) {
            return true;
        }
        n_bits_xor_k_bits[position] = 0;

        return self.set_smallest_result_bit(n_bits_xor_k_bits, position, original_pos);
    }

    fn find_differing_pos(&self, result_vec: &Vec<u8>, position: usize) -> usize {
        let mut position = position;
        while position < self.number_m_bits.len() {
            if result_vec[position] != self.number_m_bits[position] {
                return position;
            }
            position += 1;
        }
        return usize::MAX;
    }

    fn set_smallest_result_bit(&self, n_bits_xor_k_bits: &mut Vec<u8>, position: usize, lower_limit: usize) -> bool {
        let mut position = position;
        while position > lower_limit {
            if n_bits_xor_k_bits[position] == 0 && self.number_m_bits[position] == 0 {
                n_bits_xor_k_bits[position] = 1;
                return true;
            }
            position -= 1;
        }
        return false;
    }
}

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_u64_vec() -> Vec<u64> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
 }

fn main() {
    let cases_count = read_line_to_usize();
    for _ in 0..cases_count {
        let test_case = read_line_to_u64_vec();
        let bit_play = BitPlay::new(test_case[1], test_case[0]);
        let result = bit_play.compute();
        println!("{result}", result=result);
    }
}