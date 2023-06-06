use std::fs;

fn main() {
    // TODO: bad. Path is relative and hardcoded
    let file_path = "day2/input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read file");

    let mut score: i32 = 0;

    for line in contents.lines() {
        match line {
            "A X" => score = score + 4, // rock vs rock, draw + 1pt
            "A Y" => score = score + 8, // rock vs paper, win + 2pt
            "A Z" => score = score + 3, // rock vs scissors, loss + 3pt
            "B X" => score = score + 1, // paper vs rock, loss + 1pt
            "B Y" => score = score + 5, // paper vs paper, draw + 2pt
            "B Z" => score = score + 9, // paper vs scissors, win + 3pt
            "C X" => score = score + 7, // scissors vs rock, win + 1pt
            "C Y" => score = score + 2, // scissors vs paper, loss + 2pt
            "C Z" => score = score + 6, // scissors vs scissors, draw + 3pt
            _ => println!("Something went wrong"),
        }
    }
    println!("{}", score)
}
