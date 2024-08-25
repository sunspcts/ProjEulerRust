use proj_euler_rust::as_digit_vec;


fn main() {
    let mut chap_vec:Vec<u8> = vec![0];
    let mut i:u64 = 1;
    while chap_vec.len() <= 1000000 {
        chap_vec.append(&mut as_digit_vec(i).into_iter().map(|x| x as u8).collect());
        i += 1;
    }
    let prod = chap_vec[1] as u64 * chap_vec[10] as u64 * chap_vec[100] as u64 * chap_vec[1000] as u64 * chap_vec[10000] as u64 * chap_vec[100000] as u64 * chap_vec[1000000] as u64;
    println!("{}", prod)
}
