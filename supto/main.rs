/// Solution to Supto and His Teammates Problem (https://www.spoj.com/problems/SUPTO/)

use std::fmt;

struct Student {
    name: String,
    class: u32,
    score: u32,
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{name} {class} {score}", name=self.name, class=self.class, score=self.score)
    }
}

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_students_vec(students_cnt: usize) -> Vec<Student> {

    let mut students = Vec::new();
    for _ in 0..students_cnt {
        let mut string = String::new();
        let _ = std::io::stdin().read_line(&mut string);
        let student_params: Vec<&str> =  string.trim().split(' ').collect();
        let name = student_params[0].to_string();
        let class: u32 = student_params[1].parse().unwrap();
        let score: u32 = student_params[2].parse().unwrap();
        
        let student = Student {
            name: name,
            class: class,
            score: score,
        };
        students.push(student);
    }

    return students;
}

fn read_line_to_usize_vec() -> Vec<usize> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn sort_students(students: &mut Vec<Student>) {
    students.sort_by(|s1, s2| {
        if s2.score.cmp(&s1.score) != std::cmp::Ordering::Equal {
            return s2.score.cmp(&s1.score);
        }

        if s2.class.cmp(&s1.class) != std::cmp::Ordering::Equal {
            return s2.class.cmp(&s1.class);
        }

        return s1.name.cmp(&s2.name); 
    });
}

fn compute_possible_teams_combination(students: &Vec<Student>, score: usize) -> u32 {
    let mut dp = vec![vec![0; score + 1]; students.len() + 1];

    let quarter_max_u32 = std::u32::MAX / 4;
    for student_idx in 0..students.len() {
        let student_score = students[student_idx].score;

        // assumption that students are sorted in descending order
        if (student_score as usize) > score {
            continue;
        }
        for score_idx in 0..score + 1 {
            dp[student_idx + 1][score_idx] = dp[student_idx][score_idx];
            if (student_score as usize) < score_idx && student_score > 0 {
                dp[student_idx + 1][score_idx] += dp[student_idx][score_idx - student_score as usize];
            }
            
            if (student_score as usize) == score_idx {
                dp[student_idx + 1][score_idx] += 1;
            }

            if dp[student_idx + 1][score_idx] > quarter_max_u32 {
                dp[student_idx + 1][score_idx] = dp[student_idx + 1][score_idx] % 100000007;
            }

        } 
    }

    return dp[students.len()][score] % 100000007;
}

fn main() {
    let cases_count = read_line_to_usize();
    for _ in 0..cases_count {
        let n_and_k_params = read_line_to_usize_vec();
        let students_cnt = n_and_k_params[0];
        let desired_value = n_and_k_params[1];
        let mut students = read_line_to_students_vec(students_cnt);

        sort_students(&mut students);
        let teams_combinations_cnt = compute_possible_teams_combination(&students, desired_value);

        let mut students_string = String::new();
        for student_idx in 0..students.len() {
            students_string += &(students[student_idx].to_string() + "\n");
        }
        print!("{students}", students=students_string);
        println!("Number Of Ways Teams Can Be Formed = {combinations_cnt}", combinations_cnt=teams_combinations_cnt);
    }
}