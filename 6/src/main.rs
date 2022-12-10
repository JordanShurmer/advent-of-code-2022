use std::{collections::HashSet, fs::read_to_string};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).unwrap();

    let answer: [char; 4];

    let _ = input
        .chars()
        .collect::<Vec<_>>()
        .windows(4)
        .take_while(|group| {
            let unique_chars = group.iter().collect::<HashSet<_>>();
            if unique_chars.len() == 4 {
                answer = *group.try_into().unwrap();
                return false;
            }
            return true;
        });

    Ok(())
}
