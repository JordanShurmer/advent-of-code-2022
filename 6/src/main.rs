use std::{collections::HashSet, fs::read_to_string};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).unwrap();


    // a list of (index, char)
    let all_char= input.char_indices().collect::<Vec<_>>();

     let answer  = all_char

         //grouped into groups of 4
        .windows(4)

        .find_map(|group| {

            //turn each group of 4 into a HashSet
            let unique_chars = group.iter().map(|(_, c)| c).collect::<HashSet<_>>();

            return match unique_chars.len() {
                // if we have 4 elements then there's 4 unique chars
                // so return the index of the last char
                4 => Some(group.last().unwrap().0),

                // otherwise there's not
                _ => None
                
            }
        }).unwrap();

    println!("{:?}", answer+1);

    Ok(())
}
