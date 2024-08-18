use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
fn main() {
    let mut vector:Vec<Vec<u64>> = vec![];
    let lines = lines_from_file("src/bin/res/011.txt");
    for line in lines {
        let v:Vec<u64> = line.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
        vector.push(v);
    }
    let mut result:u64 = 0;
    for x in 0..17 {
        for y in 0..20 {
            let calc = horizontal(x, y, &vector);
            if calc > result {
                result = calc
            }
        }
    }
    for x in 0..20 {
        for y in 0..17 {
            let calc = vertical(x, y, &vector);
            if calc > result {
                result = calc
            }
        }
    }
    for x in 0..17 {
        for y in 0..17 {
            let calc = diagonal_1(x, y, &vector);
            if calc > result {
                result = calc
            }
        }
    }
    for x in 3..20 {
        for y in 0..17 {
            let calc = diagonal_2(x, y, &vector);
            if calc > result {
                result = calc
            }
        }
    }
    println!("{}", result)
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn horizontal(x:u64, y:u64, vector:&Vec<Vec<u64>>) -> u64 {
    let mut p:u64 = 1;
    for i in x..(x+4) {
        p *= vector[y as usize][i as usize]
    }
    p
}

fn vertical(x:u64, y:u64, vector:&Vec<Vec<u64>>) -> u64 {
    let mut p:u64 = 1;
    for i in y..(y+4) {
        p *= vector[i as usize][x as usize]
    }
    p
}

fn diagonal_1(x:u64, y:u64, vector:&Vec<Vec<u64>>) -> u64 {
    let mut p:u64 = 1;
    for i in 0..4 {
        p *= vector[(y + i) as usize][(x + i) as usize]
    }
    p
}

fn diagonal_2(x:u64, y:u64, vector:&Vec<Vec<u64>>) -> u64 {
    let mut p:u64 = 1;
    for i in 0..4 {
        p *= vector[(y + i) as usize][(x - i) as usize]
    }
    p
}