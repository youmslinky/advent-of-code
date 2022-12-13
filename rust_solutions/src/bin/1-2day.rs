//! /src/bin/1day.rs
use std::io;
/*
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
*/
fn main() {
    let mut max = 0;
    let mut current_elf_sum = 0;
    let mut all_sums:Vec<i32> = Vec::new();
    for line in io::stdin().lines()
    {
        let line_string = line.expect("read error");
        if line_string.is_empty()
        {
            all_sums.push(current_elf_sum);
            current_elf_sum = 0;
        }
        else
        {
            let line_int:i32 = line_string.parse().expect("parse error");
            current_elf_sum += line_int;
        }
        if current_elf_sum > max
        {
            max = current_elf_sum;
            println!("{max}");
        }
    }
    all_sums.push(current_elf_sum); // Add last one, because it won't necesarily have a blank line
    all_sums.sort();
    println!("sum of 3 elves with highest calories: {}", all_sums.iter().rev().take(3).sum::<i32>());
}
