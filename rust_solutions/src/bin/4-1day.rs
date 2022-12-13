use std::io::{stdin};
use std::collections::HashSet;

fn main() {
    let mut sum: u64 = 0;
    for line in stdin().lines()
    {
        let mut front_item_set: HashSet<char> = HashSet::new();
        let mut back_item_set: HashSet<char> = HashSet::new();
        let line_string = line.expect("read error");
        let line_string_length = line_string.chars().count();
        assert!( line_string_length % 2 == 0);
        for character in line_string[0..line_string_length/2].chars()
        {
            front_item_set.insert(character);
        }
        for character in line_string[line_string_length/2..].chars()
        {
            back_item_set.insert(character);
        }
        for error_character in front_item_set.intersection(&back_item_set)
        {
            sum += convert_char_to_priority(error_character) as u64;
            println!("duplicate char found: char: {} is worth {}", error_character, convert_char_to_priority(error_character));
        }
    }
    println!("sum of priorities of items: {sum}");
}
