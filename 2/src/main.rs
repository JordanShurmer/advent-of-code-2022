use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = BufReader::new(File::open(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR")))?);


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

    let part_2: HashMap<&str, i32> = [
        // loose
        ("X", 0),
        // draw
        ("Y", 3),
        // win
        ("Z", 6),

        // rocks
        ("B X", 1), //loose
        ("A Y", 1), //draw
        ("C Z", 1), //win

        // papers
        ("C X", 2), //loose
        ("B Y", 2), //draw
        ("A Z", 2), //win

        // scissors
        ("A X", 3), //loose
        ("C Y", 3), //draw
        ("B Z", 3), //win
    ].into_iter().collect();

    let (total_1, total_2) = input.lines()
        .filter_map(|line| line.ok())
        .fold((0, 0), |acc, round| {
            return (
                acc.0 + part_1[&round[2..]] + part_1.get(&round.as_str()).unwrap_or(&0),
                acc.1 + part_2[&round[2..]] + part_2.get(&round.as_str()).unwrap_or(&0)
            )
        });

    println!("part 1: {}", total_1);
    println!("part 2: {}", total_2);
    Ok(())
}
