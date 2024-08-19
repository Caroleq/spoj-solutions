// Solution to Card Trick Problem (https://www.spoj.com/problems/CTRICK/)


fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
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

    fn get_sum_in_range(&self, left_idx: usize, right_idx: usize) -> u64 {

        if left_idx > right_idx {
            return self.get_sum_in_range(left_idx, self.tree.len() - 1) + 
               self.get_sum_in_range(0, right_idx);
        }

        if left_idx == 0 {
            return self.get_sum_from_idx(right_idx);
        }
        return self.get_sum_from_idx(right_idx) - self.get_sum_from_idx(left_idx - 1);
    }

    fn len(&self) -> usize {
        return self.tree.len();
    } 

}

struct CardsOrderFinder {
    tree: FenwickTree
}

impl CardsOrderFinder {
    
    fn get_next_card_position(&mut self, start_idx: usize, target_sum: u64) -> usize {

        let mut target_sum = target_sum;
        let places_left = self.tree.len() as u64 - self.tree.get_sum_from_idx(self.tree.len()-1);
        if places_left < target_sum {
            target_sum = target_sum % places_left;
        }

        let mut left_len = target_sum as usize;
        let mut right_len = self.tree.len();
        while left_len + 1 < right_len {
            let mid_len = (left_len + right_len) / 2;

            let sum = self.tree.get_sum_in_range(start_idx, (start_idx + mid_len) % self.tree.len());

            if (mid_len as u64) < target_sum  + sum {
                left_len = mid_len;
            }
            else {
                right_len = mid_len;
            }
        }

        let left_sum = self.tree.get_sum_in_range(start_idx, (start_idx + left_len) % self.tree.len()); 

        if left_len == (target_sum + left_sum) as usize {
            return (start_idx + left_len) % self.tree.len();
        }

        return (start_idx + right_len) % self.tree.len();
    }

    fn compute_cards_vector(&mut self, cards_number: usize) -> Vec<u64> {

        let mut cards = vec![0; cards_number];

        let mut current_index = 0;
        for card_no in 0..cards_number {
            let cards_to_put_down = card_no + 1;
            let new_position = self.get_next_card_position(current_index, cards_to_put_down as u64);

            if cards[new_position] != 0 {
                continue;
            }

            cards[new_position] = card_no as u64 + 1;
            self.tree.increment(new_position);

            if card_no != cards_number - 1 {
                current_index = self.get_next_card_position(new_position, 0);
            }

        }

        return cards;
    }
    
}

fn main() {
    let test_cases_cnt = read_line_to_usize();

    for _ in 0..test_cases_cnt {
        let cards_number = read_line_to_usize();

        let mut cards_order_finder = CardsOrderFinder {
            tree: FenwickTree::new(cards_number)
        };

        let result = cards_order_finder.compute_cards_vector(cards_number);
        for card in result.iter() {
            print!("{card} ", card=*card);
        }
        println!("");
    }
}