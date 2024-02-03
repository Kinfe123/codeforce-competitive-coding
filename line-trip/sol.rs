use std::cmp;
use std::io;
fn take_int() -> i64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: i64 = input.trim().parse().unwrap();
    return input;
}
fn take_vec() -> Vec<i64> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let arr: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    return arr;
}

fn main() {
    let mut inp = take_int();
    let mut max_ = -1;
    while inp > 0 {
        let nx = take_vec();
        let n: i64 = nx[0];
        let x: i64 = nx[1];
        let points = take_vec();
        max_ = points[0];
        for indx in 1..points.len() {
            max_ = cmp::max(max_, points[indx] - points[indx - 1]);
        }
        inp -= 1;
        if let Some(&last) = points.last() {
            max_ = cmp::max(max_, (x - last) * 2);
        } else {
            println!("");
        }
        println!("{}", max_);
    }
}
