// rust implementation for waallet

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
    while inp > 0 {
        let input = take_vec();
        let a: i64 = input[0];
        let b: i64 = input[1];
        let diff = (a - b - 1).abs();

        if diff % 2 == 1 {
            println!("Bob");
        } else {
            println!("Alice");
        }
        inp -= 1;
    }
}
