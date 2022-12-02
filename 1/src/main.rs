use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();

    // Separate/Group by blank line
    let elves_calories = input.split("\n\n");


    // sum each group
    let mut totals: Vec<i32> = elves_calories.map(|elf| {
        elf.split("\n")
            .map(str::parse::<i32>)
            .filter_map(|food| food.ok())
            .sum::<i32>()
    }).collect();

    // find the highest
    totals.sort_by(|a, b| b.cmp(a));
    println!("Single highest: {}", totals.first().unwrap());
    totals.truncate(3);
    println!("Three highest: {}", totals.into_iter().sum::<i32>());

}


