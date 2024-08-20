const VALUES:[u64; 8] = [1, 2, 5, 10, 20, 50, 100, 200];

fn main() {
    let mut ways: Vec<u64> = vec![0; 201];
    ways[0] = 1;
    for x in VALUES {
        for i in x..201 {
            ways[i as usize] += ways [(i-x) as usize];
        }
    }
    println!("{}", ways[200])
}