/*
--- Day 3: Gear Ratios ---

You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.

It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.

"Aaah!"

You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.

The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.

The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

Here is an example engine schematic:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..

In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.

Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?

*/

use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    //get input from file
    // Stringify input
    let mut content = read_to_string("inputs/day3.txt").unwrap();

    // Test case:
    //content = String::from("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..");

    // Put lines into vector
    let lines: Vec<&str> = content.lines().collect();

    //get all non period non number characters from file
    let mut characters = Vec::new();
    for ch in content.chars() {
        if !ch.is_digit(10) && ch != '.' && !characters.contains(&ch) {
            characters.push(ch);
        }
    }

    // Running Total
    let mut total = 0;

    //Iterate the vector using line counts
    for (i, line) in lines.iter().enumerate() {
        // set default value for edge cases
        let top: &str = if i > 0 { lines[i - 1] } else { ".........." };
        let bottom: &str = if i < lines.len() - 1 { lines[i + 1] } else { ".........." };

        let mut num: HashMap<usize, char> = HashMap::new();
        let line_chars: Vec<char> = line.chars().into_iter().collect();
        for (j, c) in line_chars.iter().enumerate() {
            if c.is_numeric() {
                num.insert(j, *c);
            } else {
                check_number(&mut num, &characters, &line_chars, top, bottom, &mut total, line);
            }
        }

    }
    println!("{}", total);
}


fn check_number(num: &mut HashMap<usize, char>, characters: &[char], line_chars: &[char], top: &str, bottom: &str, total: &mut i32, line: &&str) {
    if !num.is_empty() {
        //Check if our number is valid
        let mut sorted_keys: Vec<usize> = num.keys().cloned().collect();
        sorted_keys.sort();
        let mut valid = false;

        // Check for symbols in the lines above or below that are touching any of the digits
        for &key in sorted_keys.iter() {
            if top.len() >= key && bottom.len() >= key {
                if (key > 0 && characters.contains(&line_chars[key - 1]))
                    || characters.contains(&line_chars[key])
                    || (key < line.len() - 1 && characters.contains(&line_chars[key + 1]))
                    || characters.contains(&top.chars().nth(key).unwrap_or_default())
                    || (key > 0 && characters.contains(&top.chars().nth(key - 1).unwrap_or_default()))
                    || (key < top.len() - 1 && characters.contains(&top.chars().nth(key + 1).unwrap_or_default()))
                    || characters.contains(&bottom.chars().nth(key).unwrap_or_default())
                    || (key > 0 && characters.contains(&bottom.chars().nth(key - 1).unwrap_or_default()))
                    || (key < bottom.len() - 1 && characters.contains(&bottom.chars().nth(key + 1).unwrap_or_default())) {
                    valid = true;
                    break;
                }
            }
        }

        if valid {
            // Combine values based on sorted keys
            let combined_string: String = sorted_keys
                .iter()
                .map(|&key| num.get(&key).unwrap())
                .collect();
            let as_int: i32 = combined_string.parse().unwrap();
            *total += as_int;
        }
        num.clear();
    }
}
// wrong: 548638, 546996

// Right: 550934