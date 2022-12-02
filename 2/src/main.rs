use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = BufReader::new(File::open("input.txt")?);


    // A,X rock
    // B,Y paper
    // C,Z scissor

    let part_1: HashMap<&str, i32> = [
        // shapes
        ("X", 1),
        ("Y", 2),
        ("Z", 3),

        // wins
        ("A Y", 6),
        ("B Z", 6),
        ("C X", 6),

        //draws
        ("A X", 3),
        ("B Y", 3),
        ("C Z", 3),
    ].into_iter().collect();

    let total = input.lines()
        .filter_map(|line| line.ok())
        .fold(0, |acc, round| {
            return acc + part_1[&round[2..]] + part_1.get(&round.as_str()).unwrap_or(&0);
        });

    println!("{}", total);
    Ok(())
}
