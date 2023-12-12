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

--- Part Two ---

The engineer finds the missing part and installs it in the engine! As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.

You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.

Before you can explain the situation, she suggests that you look out the window. There stands the engineer, holding a phone in one hand and waving with the other. You're going so slowly that you haven't even left the station. You exit the gondola.

The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of multiplying those two numbers together.

This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.

Consider the same engine schematic again:

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

In this schematic, there are two gears. The first is in the top left; it has part numbers 467 and 35, so its gear ratio is 16345. The second gear is in the lower right; its gear ratio is 451490. (The * adjacent to 617 is not a gear because it is only adjacent to one part number.) Adding up all of the gear ratios produces 467835.

What is the sum of all of the gear ratios in your engine schematic?


*/

use std::fs::read_to_string;

fn main() {
    //get input from file
    // Stringify input
    let content = read_to_string("inputs/day3.txt").unwrap();

    // Test case:
    //let content = String::from("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..");

    // Get the line length for first line
    let lines: Vec<&str> = content.lines().collect();
    let line_length = lines.get(0).map_or(0, |line| 
        line.len()) as i32;
    // After we have total line length, filter out lines. no longer needed    
    let content = content.replace('\n', "");

    // Turn String into a vector of index and char
    let characters: Vec<_> = content.chars().enumerate().collect();
    // Take that string and keep only numbers, retaining their location
    let all_nums_as_chars: Vec<(usize, char)> = characters.clone().into_iter().filter(|&(_,c)|
        c.is_numeric()
    ).collect();



    //Group neighboring numbers and retain their starting and ending locations in the characters vec
    let mut all_possible_numbers: Vec<(String, usize, usize)> = Vec::new();
    
    // Used for iterating and grouping
    let mut current = String::new();
    let mut start_index = 0;

    for (i, (index, c)) in all_nums_as_chars.iter().enumerate() {
        if i == 0 || *index == all_nums_as_chars[i -1].0 + 1{
            current.push(*c);
        } else {
            all_possible_numbers.push((current.clone(), start_index, all_nums_as_chars[i-1].0));

            current = c.to_string();
            start_index = *index;
        }
    }
    // Push last sequence
    if !current.is_empty() {
        all_possible_numbers.push((current, start_index, all_nums_as_chars.last().unwrap().0));
    }


    //Check every possible number for validity and add to total if valid
    let mut total = 0;
    for (s, start, end) in all_possible_numbers {
        if check_surroundings(start, end, line_length, &content) {
            total += s.parse::<i32>().unwrap();
        }
    }

    println!("The total of part 1 is: {}", total);

	// Part Two:
    let content = read_to_string("inputs/day3.txt").unwrap();
    //testing
    let content = String::from("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..");
    
    let gears = all_valid_gears_and_sum(content);
    println!("Gear Total (part 2): {}", gears)
    
}


// Should return true if its a valid number and false if it is not
fn check_surroundings(start: usize, end: usize, line_length: i32, content: &str) -> bool {
    for index in start..=end {
        let indices_to_check = vec![
            (index as i32 - line_length - 1) as usize, // top left
            (index as i32 - line_length) as usize,     // top
            (index as i32 - line_length + 1) as usize, // top right
            (index as i32 - 1) as usize,               // left
            (index as i32 + 1) as usize,               // right
            (index as i32 + line_length - 1) as usize, // bottom left
            (index as i32 + line_length) as usize,     // bottom
            (index as i32 + line_length + 1) as usize, // bottom right
        ];

        for check_index in indices_to_check {
            if let Some(c) = content.chars().nth(check_index) {
                if c.is_numeric() || c == '.' {
                    continue;
                } else {
                    return true;
                }
            }
        }
    }

    false
}

fn all_valid_gears_and_sum(content: String) -> i32 {
    let mut total = 0;
    let lines: Vec<&str>= content.lines().collect();    
    let line_length = lines.get(0).map_or(0, |line: &_| 
        line.len() as usize);

    let content = content.replace('\n', "");
    let chars: Vec<char> = content.chars().collect();
    for (i, c) in chars.iter().enumerate() {
        if c == &'*' {
            let mut upper: i32;
            let mut current: i32;
            let mut lower: i32;
            let upper_indices_to_check = vec![
                (i - line_length - 1) as usize, // top left
                (i - line_length) as usize,     // top
                (i - line_length + 1) as usize, // top right
            ];
            let indices_to_check = vec![
                (i - 1) as usize,               // left
                (i + 1) as usize,               // right
            ];
            let lower_indices_to_check = vec![
                (i + line_length - 1) as usize, // bottom left
                (i + line_length) as usize,     // bottom
                (i + line_length + 1) as usize, // bottom right
            ];

            for index in upper_indices_to_check.iter() {
                if chars.get(*index).map_or(false, |&ch| ch.is_digit(10)) {
                    println!("Found me a number! {}", chars[*index]);

                    //need to now check the left and right of that number for numbers, if found, continue those directions 
                }
                //println!("{}", index);
            }
        }
    }
    total
}

