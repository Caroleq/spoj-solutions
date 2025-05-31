/// Solution to Two Circles (https://www.spoj.com/problems/SMPCIRC/)

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

struct Circle {
    x_coord: i64,
    y_coord: i64,
    radius: i64
}

fn solve(circle1: &Circle, circle2: &Circle) -> String {
    
    if circle1.x_coord == circle2.x_coord &&
       circle1.y_coord == circle2.y_coord &&
       circle1.radius == circle2.radius {
        return "E".to_string();
    }

    let squared_dist = (circle1.x_coord - circle2.x_coord) * (circle1.x_coord - circle2.x_coord) + 
                        (circle1.y_coord - circle2.y_coord) * (circle1.y_coord - circle2.y_coord);
    
    let squared_radiuses = (circle1.radius + circle2.radius) * (circle1.radius + circle2.radius);

    if squared_radiuses <= squared_dist {
        return "O".to_string();
    }

    let squared_radius_diff = (circle1.radius - circle2.radius) * (circle1.radius - circle2.radius);

    if squared_dist == squared_radius_diff{
        return "E".to_string();
    }

    if squared_dist < squared_radius_diff{
        return "I".to_string();
    }
    return "O".to_string();

}

fn main() {

    let test_case_cnt = read_line_to_usize();

    for _ in 0..test_case_cnt {
        let test_case = read_line_to_i64_vec();
        let circle1 = Circle{x_coord:test_case[0], y_coord:test_case[1], radius:test_case[2]};
        let circle2 = Circle{x_coord:test_case[3], y_coord:test_case[4], radius:test_case[5]};
        let result = solve(&circle1, &circle2);
        println!("{result}", result=result);
    }
}