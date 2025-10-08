/// Solution to Coder Ratings (https://www.spoj.com/problems/RATING/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_i64_vec() -> Vec<i64> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

struct FenwickTree {
    tree: Vec<u64>
}

impl FenwickTree {

    pub fn new(tree_length: usize) -> Self {
        Self {
            tree: vec![0; tree_length]
        }
    }

    fn increment(&mut self, position: usize) {
        let mut position = position;
        while position < self.tree.len() {
            self.tree[position] += 1;
            position = position | position + 1;
        }
    }

    fn get_sum_from_idx(&self, idx: usize) -> u64 {
        let mut result = 0;
        let mut idx = idx as i64;
        
        while idx >= 0 {
            result += self.tree[idx as usize];
            idx = (idx & (idx + 1)) - 1;
        }
        return result;
    }
}

fn main() {
    let coders_cnt = read_line_to_usize();

    let mut coder_ratings = Vec::new();
    let mut second_ratings = Vec::new();
    for idx in 0..coders_cnt {
        let rating = read_line_to_i64_vec();
        coder_ratings.push((rating[0], rating[1], idx));
        second_ratings.push(rating[1]);
    }

    coder_ratings.sort_by(|e1, e2| {
        if e1.0 != e2.0 {
            return e1.0.cmp(&e2.0);
        }
        return e1.1.cmp(&e2.1);
    });
    second_ratings.sort();

    let mut fenwick_tree = FenwickTree::new(coder_ratings.len());

    let mut better_cnts = vec![0; coders_cnt];

    
    let mut position = second_ratings.binary_search(&coder_ratings[0].1).unwrap();
    fenwick_tree.increment(position);

    let mut duplicates_cnt = 0;
    for idx in 1..coders_cnt {
        position = second_ratings.binary_search(&coder_ratings[idx].1).unwrap();
        
        if coder_ratings[idx - 1].0 == coder_ratings[idx].0 && 
           coder_ratings[idx - 1].1 == coder_ratings[idx].1 { 
            duplicates_cnt += 1;
        }
        else {
            duplicates_cnt = 0;
        }
        better_cnts[coder_ratings[idx].2] = fenwick_tree.get_sum_from_idx(position) - x;
        fenwick_tree.increment(position);
    }

    for better_cnt in better_cnts {
        println!("{cnt}", cnt=better_cnt);
    } 

}