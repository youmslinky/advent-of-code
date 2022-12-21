use std::io::{stdin};
use std::collections::HashMap;
// use std::collections::VecDeque;
// use itertools::Itertools;
// use std::collections::HashSet;

fn main() {
    const WINDOW_LENGTH_TO_SEARCH_FOR: u64 = 14;
    let mut d = HashMap::new();
    for line in stdin().lines()
    {
        let line_string = line.expect("read error");
        let mut tail_iterator = line_string.chars().into_iter();
        let mut current_window_length = 0;

        for (i,c) in line_string.chars().enumerate()
        {
            increment_counter(c, &mut d);
            current_window_length += 1;
            let mut has_duplicate_char = false;
            for (_,v) in &d
            {
                // println!("key:{} value:{}",k,v);
                if v > &1
                {
                    has_duplicate_char = true;
                    break;
                }
            }
            if has_duplicate_char
            {
                let tail_character = tail_iterator.next().expect("tail character iterator error");
                decrement_counter(tail_character, &mut d);
                current_window_length -= 1;
                continue;
            }
            else
            {
                println!("found start sequence of length {current_window_length} at index (1-indexed) {}, hashmap state:{:?}", i+1, d);
                if current_window_length == WINDOW_LENGTH_TO_SEARCH_FOR
                {
                    println!("found desired window length, stopping here!");
                    break;
                }
            }
        }
    }
}

fn increment_counter(character: char, hash_map: &mut HashMap<char,u64>) -> u64
{
    if let Some(previous_count) = hash_map.remove(&character)
    {
        hash_map.insert(character, previous_count+1);
        return previous_count+1;
    }
    else
    {
        hash_map.insert(character, 1);
        return 1;
    }
}

fn decrement_counter(character: char, hash_map: &mut HashMap<char,u64>) -> u64
{
    if let Some(previous_count) = hash_map.remove(&character)
    {
        hash_map.insert(character, previous_count-1);
        return previous_count-1;
    }
    else
    {
        return 0;
    }
}
