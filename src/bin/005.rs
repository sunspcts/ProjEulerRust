fn main() {
    println!("{0}", divisible_by_all(20))
}

fn divisible_by_all(bound: u64) -> u64 {
    let mut i = 1;
    loop {
        for x in 2..bound+1{
            if i % x != 0 {i += 1; break}
            if x == 20 {return i}
        }
    }
}