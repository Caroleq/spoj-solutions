fn read_line_to_str() -> String {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string;
}

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an integer");
}
 
fn read_line_to_usize_vec() -> Vec<usize> {
    let vector_size = read_line_to_usize();

    let mut sequence = Vec::with_capacity(vector_size);
    for _ in 0..vector_size {
        sequence.push(
            read_line_to_usize()
        );
    }

    let _ = read_line_to_str();
    return sequence;
 }

 fn merge(l_seq: &Vec<usize>, r_seq: &Vec<usize>) -> (Vec<usize>, usize) {

    let mut sequence = Vec::with_capacity(l_seq.len() + r_seq.len());

    let mut i = 0;
    let mut j = 0;

    let mut inversion_count = 0;

    while i < l_seq.len() && j < r_seq.len() {

        while i < l_seq.len() && l_seq[i] <= r_seq[j] {
            sequence.push(l_seq[i]);
            i += 1;
        }

        if i == l_seq.len() {
            while j < r_seq.len() {
                sequence.push(r_seq[j]);
                j += 1;
            }
            break;
        }

        while j < r_seq.len() && r_seq[j] < l_seq[i] {
            sequence.push(r_seq[j]);
            inversion_count += l_seq.len() - i;
            j += 1;
        }
    }
    
    while i < l_seq.len() {
        sequence.push(l_seq[i]);
        i += 1;
    }

    return (sequence, inversion_count);
 }

 fn merge_sort(sequence: &Vec<usize>, begin: usize, end: usize) -> (Vec<usize>, usize)
 {
    if begin + 1 == end {
        return (Vec::from([sequence[begin]]), 0);
    }

    let mid: usize = (end + begin) / 2;
    let (l_sorted, l_cnt) = merge_sort(&sequence, begin, mid);
    let (r_sorted, r_cnt) = merge_sort(&sequence, mid, end);

    let (sorted, cnt) = merge(&l_sorted, &r_sorted);

    return (sorted, l_cnt + r_cnt + cnt);
 }

fn main() {
    let n = read_line_to_usize();
    let _ = read_line_to_str();

    for _ in 0..n {
        let sequence = read_line_to_usize_vec();

        let (_, inversion_count) = merge_sort(&sequence, 0, sequence.len());

        println!("{inversion_count}", inversion_count=inversion_count);
    }
}
