/// Solution to subarrays problem (https://www.spoj.com/problems/ARRAYSUB/)

use std::collections::BinaryHeap;

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_u32_vec() -> Vec<u32> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
}

fn compute_maximum_in_subarrays(array: &Vec<u32>, window_size: usize) -> Vec<u32> {
    let mut heap = BinaryHeap::new();
    let mut removed_elements = BinaryHeap::new();

    let mut index = 0;
    while index < window_size && index < array.len() {
        heap.push(array[index]);
        index += 1;
    }

    let mut maxima_in_windows = vec![0; array.len() - window_size + 1];
    while index < array.len() {
        maxima_in_windows[index - window_size] = *heap.peek().unwrap();

        removed_elements.push(array[index - window_size]);
        heap.push(array[index]);

        while !removed_elements.is_empty() && !heap.is_empty() && 
                *removed_elements.peek().unwrap() == *heap.peek().unwrap() {
            removed_elements.pop();
            heap.pop();
        }
        index += 1;
    }

    if heap.is_empty() {
        return maxima_in_windows;
    }

    maxima_in_windows[index - window_size] = *heap.peek().unwrap();
    return maxima_in_windows;
}

fn main() {

    let _array_len = read_line_to_usize();
    let array = read_line_to_u32_vec();
    let window_size = read_line_to_usize();

    let maximum_subarrays = compute_maximum_in_subarrays(&array, window_size);

    for maximum in maximum_subarrays {
        print!("{maximum} ", maximum=maximum);
    }
    println!("");
}