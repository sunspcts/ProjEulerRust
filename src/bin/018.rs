use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    cmp::max,
};

fn main() {
    let mut triangle:Vec<Vec<u64>> = vec![];
    let lines = lines_from_file("src/bin/res/018.txt");
    for line in lines {
        let v:Vec<u64> = line.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
        triangle.push(v);
    }
    loop {
        let rows = triangle.len();
        if rows == 1 {break};
        let mut new_row = vec![];
        let target = rows - 2;
        for i in 0..target+1 {
            new_row.push(triangle[target][i] + max(triangle[target+1][i], triangle[target+1][i+1]))
        }
        triangle = triangle[0..triangle.len()-2].to_vec();
        triangle.push(new_row)
    }
    println!("{}", triangle[0][0])
    
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}