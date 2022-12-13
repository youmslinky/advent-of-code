use std::io;
use std::collections::HashSet;

fn main() {
    let mut sum: u64 = 0;
    for line in io::stdin().lines()
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

fn convert_char_to_priority(character: &char) -> u8
{
    match character
    {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => todo!(),
    }
}

