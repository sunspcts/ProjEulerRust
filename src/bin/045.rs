fn main() {
    let mut p = 166; let mut h = 144; let mut pi = pentagon(p); let mut hi = hexagon(h);
    while pi != hi {
        if pi < hi {pi = pentagon(p); p += 1}
        else {hi = hexagon(h); h += 1}
    }
    println!("{0}, {1}", pi, hi)
}

fn pentagon(n:u64) -> u64 {
    (3 * n * n - n) / 2
}

fn hexagon(n:u64) -> u64 {
    n * (2 * n - 1)
}