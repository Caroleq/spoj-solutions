/// Solution to Counting Triangles Problem (https://www.spoj.com/problems/TRICOUNT/)
/// Formula taken from https://math.stackexchange.com/questions/203873/how-many-triangles

fn rl() -> usize {
    let mut st = String::new();
    let _ = std::io::stdin().read_line(&mut st);
    return st.trim().parse().expect("not usize");
}

fn main() {
    let cnt = rl();

    for _ in 0..cnt {
        let n = rl();
        let mut r = (n * (n + 1) * (n + 2)) / 6;
        if n % 2 == 0 { r += (n * (n + 2) * (2 * n - 1)) / 24; }
        else { r += ((n - 1) * (n + 1) * (2 * n + 3)) / 24; }
    
        println!("{r}", r=r);
    }
}