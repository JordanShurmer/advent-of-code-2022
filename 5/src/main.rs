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
        .rev()
        .flat_map(|line| line.char_indices().filter(|(_, c)| c.is_alphabetic()))

        // account for the '[ ]'
        .map(|(i, c)|(((i - 1) / 4) + 1, c))

        .fold(HashMap::new(), |mut stacks, row| {
            let stack = stacks.entry(row.0).or_default();
            stack.push(row.1);
            stacks
        });

    // input
    //     .filter(|line| line.starts_with("move"))
    //     .map(|line| line.split_ascii_whitespace().filter_map(|s| s.parse::<usize>().ok()).collect::<Vec<_>>())
    //     .for_each(|action| {
    //         let (count, source, destination) = (action[0], action[1], action[2]);
    //         let mut source_stack = all_stacks.get(&source).unwrap();
    //         let mut dest_stack = all_stacks.get(&destination).unwrap();
    //
    //         let mut move_these = source_stack.split_off(source_stack.len() - count);
    //         dest_stack.append(&mut move_these)
    //     });

    println!("{:?}", all_stacks);
    Ok(())
}
