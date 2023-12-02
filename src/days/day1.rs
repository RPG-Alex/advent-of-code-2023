use core::num;
/*

--- Day 1: Trebuchet?! ---

Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.

You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").

As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.

The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet

In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

Consider your entire calibration document. What is the sum of all of the calibration values?

Your puzzle answer was 55538.

The first half of this puzzle is complete! It provides one gold star: *
--- Part Two ---

Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

Equipped with this new information, you now need to find the real first and last digit on each line. For example:

two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen

In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.

What is the sum of all of the calibration values?


*/
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
	//get input from file
	let file = File::open("day1_input.txt");

	// Stringify input
	let mut content = String::new();
	let _ = file.expect("failed to read file").read_to_string(&mut content);

	// Part 1
	let mut total = 0;
	for line in content.split_whitespace() {
		let first_digit = line.chars().filter(|n| n.is_numeric()).next();
		let last_digit = line.chars().rev().filter(|n| n.is_numeric()).next();
		let mut combined = String::from(first_digit.unwrap());
		combined.push(last_digit.unwrap());
		
		total += combined.parse::<i32>().unwrap() as i32;
		
	}
	println!("part 1: {}", total);

	// Part 2
	let mut total = 0;
	let mut map = HashMap::new();
		map.insert("one", "1");
		map.insert("two", "2");
		map.insert("three", "3");
		map.insert("four", "4");
		map.insert("five", "5");
		map.insert("six", "6");
		map.insert("seven", "7");
		map.insert("eight", "8");
		map.insert("nine", "9");

	for line in content.split_whitespace() {
		let mut number = String::new();
		let mut word = String::new();

		for char in line.chars() {
			if char.is_numeric(){
				number.push(char);
				word.clear();
				break;
			} else {
				word.push(char);

				for (key, _) in &map {
					if word.contains(key) {
						word = key.to_string();
					}
				}

				if map.contains_key(word.as_str()) {
					if let Some (value) = map.get(word.as_str()) {
						number.push_str(value);
					}
					word.clear();
					break;
				}
			}
		}

		for char in line.chars().rev() {
			if char.is_numeric(){
				number.push(char);
				word.clear();
				break;
			} else {
				word.push(char);
				let mut drow: String = word.chars().rev().collect();
				for (key, _) in &map {
					if drow.contains(key) {
						drow = key.to_string();
					}
				}

				if map.contains_key(drow.as_str()) {
					if let Some (value) = map.get(drow.as_str()) {
						number.push_str(value);
					}
					word.clear();
					break;
				}
			}
		}
		if let Ok(num) = number.parse::<i32>() {
			total += num;
		}
	}
	println!("part 2: {}", total);

}
