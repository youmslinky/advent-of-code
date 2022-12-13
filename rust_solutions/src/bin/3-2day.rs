use std::io::{stdin, BufRead};
use std::collections::HashSet;

fn main() {
    let mut sum: u64 = 0;
    let mut lines = stdin().lock().lines();
    while let Some(sack1) = lines.next()
    {
        let sack1_string = sack1.unwrap();
        let sack2_string = lines.next().unwrap().unwrap();
        let sack3_string = lines.next().unwrap().unwrap();
        // println!("{} {} {}", sack1_string, sack2_string, sack3_string);
        let mut sack1_set: HashSet<char> = HashSet::new();
        let mut sack2_set: HashSet<char> = HashSet::new();
        let mut sack3_set: HashSet<char> = HashSet::new();
        for character in sack1_string.chars()
        {
            sack1_set.insert(character);
        }
        for character in sack2_string.chars()
        {
            sack2_set.insert(character);
        }
        for character in sack3_string.chars()
        {
            sack3_set.insert(character);
        }

        let mut sack1_sack2_intersection: HashSet<char> = HashSet::new();
        for character in sack1_set.intersection(&sack2_set)
        {
            sack1_sack2_intersection.insert(*character);
        }
        let mut count = 0;
        for error_character in sack1_sack2_intersection.intersection(&sack3_set)
        {
            sum += convert_char_to_priority(error_character) as u64;
            println!("duplicate char found: char: {} is worth {}", error_character, convert_char_to_priority(error_character));
            if count >= 1
            {
                panic!();
            }
            count += 1
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

