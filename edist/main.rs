/// Solution to Edit distance (EDIST) problem: https://www.spoj.com/problems/EDIST/

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line() -> Vec<u8> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().as_bytes().to_vec();
}

fn compute_edit_distance(A: &Vec<u8>, B: &Vec<u8>) -> usize {
    let n = A.len();
    let m = B.len();

    if n == 0 {
        return m;
    }

    if m == 0 {
        return n;
    }

    let mut dp = vec![vec![0; m + 1]; n + 1];

    for i in 0..(n+1) {
        dp[i][0] = i;
    }

    for j in 0..(m+1) {
        dp[0][j] = j;
    }

    for i in 1..(n+1) {
        for j in 1..(m+1) {
            if A[i - 1] == B[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            }
            else {
                dp[i][j] = 1 + std::cmp::min(
                    std::cmp::min(dp[i - 1][j], dp[i][j - 1]),
                    dp[i - 1][j - 1]
                );
            }
        }
    }

    return dp[n][m];
}


fn main() {

    let n = read_line_to_usize();
    for _ in 0..n {
        let A = read_line();
        let B = read_line();

        let edist = compute_edit_distance(&A, &B);
        println!("{edist}", edist=edist);
    }
}