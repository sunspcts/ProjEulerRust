use itertools::Itertools;

fn main() {
    let perms: itertools::Permutations<std::ops::Range<u64>> = (0..10).permutations(10);
    let mut string_perms:Vec<String> = vec!{};
    for i in perms {
        string_perms.push(i.into_iter().map(|x| x.to_string()).collect::<String>());
    }
    string_perms.sort();
    println!("{}", string_perms[999999]);
}