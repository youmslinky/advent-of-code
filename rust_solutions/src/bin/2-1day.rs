//! /src/bin/1day.rs
use std::io;

fn main() {
    let mut sum = 0;
    for line in io::stdin().lines()
    {
        let line_string = line.expect("read error");
        let mut split = line_string.split_whitespace();
        let pair = (split.next().unwrap(), split.next().unwrap());
        let single_score_result = calculate_my_score_from_1_throw(pair);
        sum += single_score_result;
    }
    // println!("sum of 3 elves with highest calories: {blah}");
    println!("score from secret sheet: {sum}");
}

fn calculate_my_score_from_1_throw(throw: (&str, &str)) -> i32
{
    let translation = match throw
    {
        ("A", "X") => ("R", "R"),
        ("B", "X") => ("P", "R"),
        ("C", "X") => ("S", "R"),
        ("A", "Y") => ("R", "P"),
        ("B", "Y") => ("P", "P"),
        ("C", "Y") => ("S", "P"),
        ("A", "Z") => ("R", "S"),
        ("B", "Z") => ("P", "S"),
        ("C", "Z") => ("S", "S"),
        (&_, _) => todo!(), //panic if we get a bad value
    };
    match translation
    {
        ("R", "R") => 1+3,
        ("P", "R") => 1+0,
        ("S", "R") => 1+6,
        ("R", "P") => 2+6,
        ("P", "P") => 2+3,
        ("S", "P") => 2+0,
        ("R", "S") => 3+0,
        ("P", "S") => 3+6,
        ("S", "S") => 3+3,
        (&_, _) => todo!(), //panic if we get a bad value
    }
}
