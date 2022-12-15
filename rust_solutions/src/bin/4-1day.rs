use std::io::{stdin};
use itertools::Itertools;
// use std::collections::HashSet;

fn main() {
    let mut completely_overlapping_sets_count = 0;
    for line in stdin().lines()
    {
        let line_string = line.expect("read error");
        // let elf_work_order_pair: Vec<&str> = line_string.split([',','-']).collect();
        println!("{line_string}");
        //TODO convert strings to ints LOLOLOL
        // let line_int:i32 = line_string.parse().expect("parse error");
        let (elf1_lower, elf1_upper, elf2_lower, elf2_upper): (u64, u64, u64, u64) = line_string.split([',','-']).map(|s| s.parse().unwrap()).collect_tuple().unwrap();
        if (elf1_lower <= elf2_lower) && (elf2_upper <= elf1_upper)
        {
            println!("({elf1_lower} <= {elf2_lower}) && ({elf2_upper} <= {elf1_upper})");
            completely_overlapping_sets_count += 1;
            println!("condition 1 met");
        }
        else if (elf2_lower <= elf1_lower) && (elf1_upper <= elf2_upper)
        {
            println!("{elf2_lower} <= {elf1_lower} && {elf1_upper} <= {elf2_upper}");
            completely_overlapping_sets_count += 1;
            println!("rONDITION 2 MET");
        }
        // let single_elf_work_range: Vec<&str> = elf_work_order_pair.iter().map(|s| s.split("-")).collect();
        // dbg!(single_elf_work_range);
        // println!("split string: {}", split);
        println!("");
    }
    // println!("sum of priorities of items: {sum}");
    println!("completely overlapping sets count: {completely_overlapping_sets_count}");
}
