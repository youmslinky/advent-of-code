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
    for line in io::stdin().lines()
    {
        let line_string = line.expect("read error");
        if line_string.is_empty()
        {
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
    println!("elf with max calories: {max}");
}
