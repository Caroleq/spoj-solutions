fn rl()->usize{
let mut s=String::new();
std::io::stdin().read_line(&mut s);
s.trim().parse().expect("")
}

type Vf=Vec<f64>;
type VVf=Vec<Vf>;
type Vi=Vec<i64>;

fn fft(p:&VVf,lut:&VVf)->VVf{
let l=p.len();
if l==1{return p.to_vec();}

let mut e=vec![vec![0.0,0.0];l/2];
let mut o=e.clone();

for i in 0..l/2{
e[i]=vec![p[i*2][0],p[i*2][1]];
o[i]=vec![p[i*2+1][0],p[i*2+1][1]];
}

let e=fft(&e,lut);
let ov=fft(&o,lut);

let mut r=vec![vec![0.0,0.0];l];
for i in 0..l/2{
let p=mul(&lut[i*lut.len()/l],&ov[i]);
r[i]=vec![&e[i][0]+p[0],&e[i][1]+p[1]];
r[i+l/2]=vec![&e[i][0]-p[0],&e[i][1]-p[1]];
}
r
}

fn cf(p:&Vi,dg:usize)->VVf{
let mut n=1;
while n<2*(dg+1){n<<=1;}
let mut cp=vec![vec![0.0,0.0];n];
for i in 0..p.len(){cp[i]=vec![p[p.len()-1-i] as f64,0.0];}
cp
}

fn mul(a:&Vf,b:&Vf)->Vf{vec![a[0]*b[0]-a[1]*b[1],a[1]*b[0]+a[0]*b[1]]}

fn nr(p:&Vi)->Vi{
let mut r=Vec::new();
let mut c=0;
for i in (0..p.len()).rev(){
c+=p[i];r.push(c%10);c/=10;
}
while c>0{
r.push(c%10);c/=10;
}
while r[r.len()-1]==0{r.pop();}
r.reverse();
r
} 

fn main(){
let cnt=rl();
let mut fs=vec![vec![1];1];

let n=2097152;
let mut lt=vec![vec![0.0,0.0];n];
let en=std::f64::consts::PI*2.0/n as f64;
let w_n=vec![en.cos(),en.sin()];
let mut o=vec![1.0,0.0];

for i in 0..n{
lt[i]=o.to_vec();o=mul(&w_n,&o);
}
let li:Vec<_>=lt.clone().into_iter().rev().collect();

for fv in 2..101{
let nfcp: Vec<_>= fv.to_string().chars().map(|c| c.to_digit(10).unwrap() as i64).collect();
let dg=fs[fv-2].len()+nfcp.len();
let cp1=cf(&nfcp,dg);
let cp2=cf(&fs[fv-2],dg);
let l=cp1.len();
let v1=fft(&cp1,&lt);
let v2=fft(&cp2,&lt);
let mut mv=vec![vec![0.0,0.0];l];
for i in 0..l{mv[i]=mul(&v1[i],&v2[i]);}
let mp=fft(&mv,&li);
let mut r=vec![0;l];
for i in 0..l{r[l-i-1]=((mp[i][0]+0.7)/l as f64)as i64;}
fs.push(nr(&r));
}

for _ in 0..cnt{println!("{s}",s=fs[rl()-1].clone().into_iter().map(|i|i.to_string()).collect::<String>());}
}