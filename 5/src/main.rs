use std::collections::HashMap;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = std::io::BufReader::new(
        std::fs::File::open(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR")))?
    ).lines().filter_map(|line| line.ok());

    let stack_specification = input.by_ref()
        // stop at the bottom of the stack def
        .take_while(|line| !line.starts_with(" 1 "))
        .collect::<Vec<_>>();

    let all_stacks: HashMap<usize, Vec<_>> = stack_specification.iter()
        // start at the bottom of the stacks
        .rev()
        // .inspect(|line| println!("{}", line))

        .flat_map(|line| line.char_indices().filter(|(_, c)| c.is_alphabetic()))
        .map(|(i, c)| {
            // account for the '[ ]'
            let crate_num = match i {
                1 => 1,
                _ => ((i - 1) / 4) + 1
            };
            return (crate_num, c);
        })
        .fold(HashMap::new(), |mut stacks, row| {
            let stack = stacks.entry(row.0).or_default();
            stack.push(row.1);
            stacks
        });

    println!("{:?}", all_stacks);
    Ok(())
}
