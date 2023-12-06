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
use std::fs::File;
use std::io::Read;

fn main() {
        //get input from file
	let file = File::open("inputs/day3.txt");

	// Stringify input
	let mut content = String::new();
	let _ = file.expect("failed to read file").read_to_string(&mut content);

    // Test case:
    content = String::from("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..");

    // Put lines into vector
    let lines: Vec<&str> = content.lines().collect();

    let characters = [
        '*',
        '#',
        '+',
        '$',
        '&',
        '=',
        '/',
        '%',
        '@',
        '!',
        '^',
        '(',
        ')',
        '-',
    ];

    // Create a counter to track the total
    let mut total = 0;

    //Iterate the vector using line counts
    for (i, line) in lines.iter().enumerate() {
        // set default value for edge cases
        let mut top = "..........";
        let mut bottom = "..........";
        
        // If top or bottom exist, swap for actual lines
        if i > 0 {
            top = lines.get(i-1).expect("Unable to locate previous line");
        }
        if i < lines.len()-1 {
            bottom = lines.get(i+1).expect("Unable to get next line");
        }
        
        let mut sub_total = 0;
        let mut num = String::new();
        let line_chars: Vec<char> = line.chars().into_iter().collect();
        for (j, c) in line_chars.iter().enumerate() {
            if c.is_numeric() {
                if characters.iter().any(|&symbol|  {
                    let neighbors = [
                        
                        top.chars().nth(j).unwrap(),
                        if j > 0 {
                            top.chars().nth(j-1).unwrap_or(' ')
                        } else {
                            ' '
                        },
                        top.chars().nth(j+1).unwrap_or(' '),
                        line.chars().nth(j.wrapping_sub(1)).unwrap_or(' '),
                        line.chars().nth(j+1).unwrap_or(' '),
                        bottom.chars().nth(j).unwrap(),
                        if j > 0 {
                            bottom.chars().nth(j-1).unwrap_or(' ')
                        } else {
                            ' '
                        },
                        bottom.chars().nth(j+1).unwrap_or(' '),
                    ];
                    neighbors.contains(&symbol)
                }) {
                    num.push(*c);
                } else if !num.is_empty() {
                    num.push(*c);
                } else {
                    if !num.is_empty() {
                        sub_total += num.parse::<i32>().unwrap();
                        //num.clear();    
                    }
                    
                }
            }
        }
        
        println!("{:#?}", num);

        //println!("{:#?}", line_chars);
    //     for (j, c) in line_chars {
    //     if c.is_numeric() {
    //         num.push(c);
    //         if characters.iter().any(|&symbol|  {
    //             let neighbors = [
                    
    //                 top.chars().nth(j).unwrap(),
    //                 if j > 0 {
    //                     top.chars().nth(j-1).unwrap_or(' ')
    //                 } else {
    //                     ' '
    //                 },
    //                 top.chars().nth(j+1).unwrap_or(' '),
    //                 line.chars().nth(j.wrapping_sub(1)).unwrap_or(' '),
    //                 line.chars().nth(j+1).unwrap_or(' '),
    //                 bottom.chars().nth(j).unwrap(),
    //                 if j > 0 {
    //                     bottom.chars().nth(j-1).unwrap_or(' ')
    //                 } else {
    //                     ' '
    //                 },
    //                 bottom.chars().nth(j+1).unwrap_or(' '),
    //             ];
    //             neighbors.contains(&symbol)
    //         }) {
                
    //             num.push(c);
                
    //             print!("{}\n",sub_total);
    //         } else {
    //             num.clear();
    //         }
    //     }
    //    }

    }
}