/// Solution to Build a Fence problem (https://www.spoj.com/problems/FENCE1/)


fn read_line_to_u32() -> u32 {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned u8 integer");
}

fn main() {
    let pi = 3.1415926536897932384626;
    loop {
        let fence_length = read_line_to_u32();
        if fence_length == 0 {
            break;
        }

        let maximum_area = (fence_length * fence_length) as f32 / (2.0 * pi);

        println!("{maximum_area:.2}", maximum_area=maximum_area);
    }
}