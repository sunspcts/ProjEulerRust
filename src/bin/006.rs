//ez

fn main() {
    println!("{0}", squareofsum(100) - sumofsquares(100));
}

fn sumofsquares(bound: u64) -> u64 {
    let mut s = 0;
    for i in 1..bound+1 {
        s += i * i
    }
    s
}

fn squareofsum(bound: u64) -> u64 {
    let mut s = 0;
    for i in 1..bound+1 {
        s += i
    }
    s * s
}