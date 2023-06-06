use std::fs;


fn main() {
    // TODO: bad. Path is relative and hardcoded
    let file_path = "day1/input.txt";
    
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read file");


    let mut biggest: i32 = 0;
    let mut counter: i32 = 0;
    // TODO: there's probably a way to do this with a fixes size array?
    let mut top_three: Vec<i32> = Vec::new();
    for line in contents.lines() {
        if line != "" {
            counter = counter + line.parse::<i32>().unwrap();
        } 
        else {
           top_three.push(counter);
           biggest = std::cmp::max(biggest, counter);
           counter = 0;
        }
    }
    top_three.sort();
    let len = top_three.len();
    // TODO: this would panic if there are less than 3 elements in the Vec
    let tot = top_three[len-1] + top_three[len-2] + top_three[len-3];
    println!("The elf holding the most calories holds {} calories.", biggest);
    println!("#1 is holding {:?} calories", top_three[len-1]);
    println!("#2 is holding {:?} calories", top_three[len-2]);
    println!("#3 is holding {:?} calories", top_three[len-3]);
    println!("The top 3 elves are holding {} calories", tot)

}
