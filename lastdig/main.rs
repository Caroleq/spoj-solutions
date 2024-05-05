/// Solution to The last digit Problem (https://www.spoj.com/problems/LASTDIG/)

fn read() -> Vec<u64>{
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    s.trim().split(' ').map(|s| s.parse().unwrap()).collect()
}

fn compute(base: u64, exponent: u64) -> u64{

    if exponent==0{return 1;}

    let l = base%10;
    if exponent%4==0{
        if l==7||l==3||l==9 {1}
        else if l==5||l==6||l==0||l==1 {l}
        else {6}
    }
    else {base.pow((exponent%4) as u32)%10}
}

fn main(){
    let t = read()[0];

    for _ in 0..t{
        let t = read();
        let b = t[0];
        let e = t[1];
        let r = compute(b, e);
        println!("{r}", r=r);
    }
}