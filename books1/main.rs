// Solution to Copying Books problem (https://www.spoj.com/problems/BOOKS1/)

fn read_line_to_size() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_u64_vec() -> Vec<u64> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn can_have_every_scribe_max_sum_pages(books_pages: &Vec<u64>, max_sum: u64, scribes_cnt: usize) -> bool {

    let mut current_part_idx = 0;
    let mut current_sum = 0;
    for book in books_pages {

        if *book > max_sum {
            return false;
        }

        if current_sum + *book > max_sum {
            current_part_idx += 1;
            current_sum = 0;
            if current_part_idx == scribes_cnt {
                return false;
            }
        }

        current_sum += book;
    }

    return true;
}

fn generate_division_indexes(books_pages: &Vec<u64>, max_sum: u64, scribes_cnt: usize) -> Vec<usize> {

    let mut result = Vec::new();

    let mut current_sum = 0;
    for book_idx in (0..books_pages.len()).rev() {

        if result.len() + 1 == scribes_cnt as usize{
            break;
        }
        
        if book_idx as u64  == scribes_cnt as u64 - result.len() as u64 - 2 {
            let mut book_idx = book_idx as i64;
            while book_idx > -1 {
                result.push(book_idx as usize);
                book_idx -= 1;
            }
            result.reverse();
            return result;
        }

        if current_sum + books_pages[book_idx] > max_sum {
            current_sum = 0;
            result.push(book_idx);
        }

        current_sum += books_pages[book_idx];
    }

    result.reverse();
    return result;
}

fn compute_min_book_copying_work(books_pages: &Vec<u64>, scribes_cnt: usize) -> Vec<usize> {

    let mut left_min_sum: u64 = *books_pages.iter().min().unwrap();
    let mut right_min_sum: u64 = books_pages.iter().sum();

    while left_min_sum + 2 < right_min_sum {
        let min_sum = (left_min_sum + right_min_sum) / 2;
        if can_have_every_scribe_max_sum_pages(books_pages, min_sum, scribes_cnt){
            right_min_sum = min_sum;
        }
        else {
            left_min_sum = min_sum;
        }
    }

    let mut min_sum = right_min_sum;
    if can_have_every_scribe_max_sum_pages(books_pages, left_min_sum, scribes_cnt) {
        min_sum = left_min_sum;
    }
    else if can_have_every_scribe_max_sum_pages(books_pages, left_min_sum + 1, scribes_cnt) {
        min_sum = left_min_sum + 1;
    }
    
    return generate_division_indexes(books_pages, min_sum, scribes_cnt);
}

fn main() {

    let cases_cnt = read_line_to_size();
    for _ in 0..cases_cnt {
        let scribes_cnt = read_line_to_u64_vec()[1] as usize;
        let books_pages = read_line_to_u64_vec();

        let mut result = String::new();
        let book_ranges = compute_min_book_copying_work(&books_pages, scribes_cnt);

        let mut current_page_idx = 0;

        for book_idx in 0..books_pages.len() {
            result += &books_pages[book_idx].to_string();
            if current_page_idx < book_ranges.len() && book_ranges[current_page_idx] == book_idx {
                result += " / ";
                current_page_idx += 1;
            }
            else {
                result += " "; 
            }
        }

        println!("{result}", result=result);
    }
}
