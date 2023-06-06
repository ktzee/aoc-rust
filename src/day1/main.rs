use std::fs;


fn main() {
    let file_path = "day1/input.txt";
    
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read file");


    let mut biggest: i32 = 0;
    let mut counter: i32 = 0;
    for line in contents.lines() {
        if line != "" {
            counter = counter + line.parse::<i32>().unwrap();
        } 
        else {
           biggest = std::cmp::max(biggest, counter);
           counter = 0;
        }
    }

    println!("The elf holding the most calories holds {} calories.", biggest)

}
