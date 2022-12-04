#![feature(iter_array_chunks)]
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = BufReader::new(File::open(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR")))?);

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


    let input = BufReader::new(File::open(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR")))?);
    let part_2: u32 = input.lines()
        .filter_map(|line| line.ok())

        // groups of 3 at a time
        .array_chunks::<3>()

        // each elf as a set of chars
        .map(|group| group.map(|s| s.chars().collect::<HashSet<_>>()))

        // intersect of all 3
        .flat_map(|group| &(&group[0] & &group[1]) & &group[2])

        //compute the score
        .map(score)
        .sum();


    println!("part_2 total: {}", part_2);

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