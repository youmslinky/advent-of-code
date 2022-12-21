use std::io::{stdin};
use std::collections::VecDeque;
// use itertools::Itertools;
// use std::collections::HashSet;

fn main() {
    // let mut box_stacks: Vec<Vec<char>> = vec![Vec::new(); 9];
    // let mut box_stack_initilized = false;
    let mut d: VecDeque<char> = VecDeque::new();
    for line in stdin().lines()
    {
        let line_string = line.expect("read error");
        for (i,c) in line_string.chars().enumerate()
        {
            d.push_back(c);
            if i >= 4
            {
                d.pop_front();
                // dbg!(i,c,&d);
                if d[0] == d[1] || d[0] == d[2] || d[0] == d[3]
                {
                }
                else if d[1] == d[2] || d[1] == d[3]
                {
                }
                else if d[2] == d[3]
                {
                }
                else
                {
                    println!("found start sequence at index (1-indexed) {}, deque state:{:?}", i+1, d);
                    break;
                }
            }
            // println!();
        }
    }
}
