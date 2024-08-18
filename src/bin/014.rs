use::proj_euler_rust::Collatz;

fn main() {
    let mut maximum = [0, 0];
    for i in 1..1000000 {
        let count: u64 = Collatz{curr: i}.count() as u64;
        if count > maximum[1] {
            maximum = [i, count]
        }
    }
    println!("{:?}", maximum)
}