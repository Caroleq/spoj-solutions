// Solution to PIE Problem (https://www.spoj.com/problems/PIE/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_f64_vec() -> Vec<f64> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn read_line_to_usize_vec() -> Vec<usize> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn compute_volumes(radii: &Vec<f64>) -> Vec<f64> {
    let mut volumes = vec![0.0; radii.len()];

    let pi = 3.141592653589793238;

    for radius_idx in 0..radii.len() {
        volumes[radius_idx] = radii[radius_idx] * radii[radius_idx] * pi;
    }

    return volumes;
}

fn can_assign_pie_volume(target_pie_volume_per_friend: f64, pie_volumes: &Vec<f64>, person_cnt: usize) -> bool {

    let mut friends_who_got_pie = 0;

    for volume in pie_volumes {

        friends_who_got_pie += (*volume / target_pie_volume_per_friend) as usize;
        if friends_who_got_pie >= person_cnt {
            return true;
        }
    }

    return false;
}

fn compute_maximum_pie(pie_volumes: &Vec<f64>, person_cnt: usize) -> f64 {

    let mut max_pie_volume_per_friend = *pie_volumes.into_iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(); 
    let mut min_pie_volume_per_friend = *pie_volumes.into_iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap() / person_cnt as f64;

    let accuracy = 0.00001;
    while max_pie_volume_per_friend - min_pie_volume_per_friend > accuracy {
       let target_pie_volume_per_friend = (max_pie_volume_per_friend + min_pie_volume_per_friend) / 2.0;

       if can_assign_pie_volume(target_pie_volume_per_friend, pie_volumes, person_cnt) {
        min_pie_volume_per_friend = target_pie_volume_per_friend;
       }
       else {
        max_pie_volume_per_friend = target_pie_volume_per_friend;
       }
    }

    return min_pie_volume_per_friend;
}

fn main() {
    let test_cases_cnt = read_line_to_usize();

    for _ in 0..test_cases_cnt {
        let friends_cnt = read_line_to_usize_vec()[1];
        let pies_radii = read_line_to_f64_vec();

        let pie_volumes = compute_volumes(&pies_radii);
        let max_pie_per_person = compute_maximum_pie(&pie_volumes, friends_cnt + 1);
        
        println!("{max_pie_per_person}", max_pie_per_person=max_pie_per_person);
    }
}