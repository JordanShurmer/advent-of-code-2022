use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = BufReader::new(File::open("input.txt")?);

    let part_1: u32 = input.lines()
        .filter_map(|line| line.ok())

        // split the rucksack into equal compartments
        .map(|rucksack| {
            let (compartment_1, compartment_2) = rucksack.split_at(rucksack.len() / 2);
            // not sure why I cant just return the .split_at directly.
            // spent to long trying things.. went with the explicit thing here
            (compartment_1.to_owned(), compartment_2.to_owned())
        })

        // get the chars of each compartment, in a Set
        .map(|(compartment_1, compartment_2)| (
            compartment_1.chars().collect::<HashSet<_>>(),
            compartment_2.chars().collect::<HashSet<_>>()
        ))

        // .inspect(|thing| println!("{:?}", thing))

        // get the intersection of both compartments
        .flat_map(|(set_1, set_2)| &set_1 & &set_2)

        // .inspect(|thing| println!("{:?}", thing))

        // compute the score of all combined
        .map(score)

        .sum();

    println!("part_1 total: {}", part_1);

    Ok(())
}


fn score(char: char) -> u32 {
    if char.is_uppercase() {
        // A-Z is 27-52
        return (char as u32) - ('A' as u32) + 27;
    }
    // a-z is 1-26
    return (char as u32) - ('a' as u32) + 1;
}

#[test]
fn scores() {
    assert_eq!(score('A'), 27);
    assert_eq!(score('B'), 28);
    assert_eq!(score('Z'), 52);
    assert_eq!(score('a'), 1);
    assert_eq!(score('b'), 2);
    assert_eq!(score('z'), 26);
}