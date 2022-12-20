use std::io::{stdin};
// use itertools::Itertools;
// use std::collections::HashSet;

fn main() {
    let mut box_stacks: Vec<Vec<char>> = vec![Vec::new(); 9];
    let mut box_stack_initilized = false;
    for line in stdin().lines()
    {
        let mut box_stack_index = 0;
        let line_string = line.expect("read error");
        if !box_stack_initilized
        {
            for (i,c) in line_string.chars().enumerate()
            {
                if c.is_digit(10)
                {
                    box_stack_initilized = true;
                    break;
                }
                if i%4 == 1
                {
                    if c.is_alphabetic()
                    {
                        // println!("index:{i} char:{c} box_stack_index:{box_stack_index}");
                        box_stacks[box_stack_index].push(c);
                        // box_stacks.iter_mut().for_each(|v| println!("{:?}", v));
                        // println!();
                    }
                    box_stack_index += 1;
                }
                else
                {
                    // println!("index:{i} char:{c}");
                }
            }
        }
        else
        {
            // done with getting intial box state, break out of loop
            box_stacks.iter_mut().for_each(|v| v.reverse()); //reverse each vec in place
            box_stacks.iter_mut().for_each(|v| println!("{:?}", v)); //print each vec
            break;
        }
    }
    // loop over rest of lines to get crane instructions
    for line in stdin().lines()
    {
        let line_string = line.expect("read error");
        let line_split: Vec<&str> = line_string.split(" ").collect();
        let number_of_boxes_to_move = line_split[1].parse().unwrap();
        let stack_origin_index: usize = line_split[3].parse::<usize>().unwrap() - 1;
        let stack_destination_index: usize = line_split[5].parse::<usize>().unwrap() - 1;

        // stack boxes into temp stack
        let temporary_stack: &mut Vec<char> = &mut Vec::new();
        for _ in 0..number_of_boxes_to_move
        {
            let origin_stack: &mut Vec<char> = &mut box_stacks[stack_origin_index];
            let popped_value: &char = &origin_stack.pop().unwrap();
            temporary_stack.push(*popped_value);
        }
        // unload temp stack into destination stack
        for _ in 0..number_of_boxes_to_move
        {
            let popped_value: &char = &temporary_stack.pop().unwrap();
            let destination_stack: &mut Vec<char> = &mut box_stacks[stack_destination_index];
            destination_stack.push(*popped_value);
        }
        box_stacks.iter_mut().for_each(|v| println!("{:?}", v)); //print each vec
        println!("instructon: {}, {}, {}", number_of_boxes_to_move, stack_origin_index, stack_destination_index);
        println!();
        // let instruction_tuple: (u64, u64, u64) = line_string.split(" ").map(|s| s.parse().unwrap()).collect_tuple().unwrap();
    }
    box_stacks.iter_mut().for_each(|v| v.reverse()); //reverse each vec in place
    box_stacks.iter_mut().for_each(|v| print!("{}", v[0])); //print last char in each vec
    println!();
}
