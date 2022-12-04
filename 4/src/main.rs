use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = BufReader::new(File::open(format!(
        "{}/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?);

    let count = input
        .lines()
        .filter_map(|line| line.ok())

        // turn the strings into "Range"s
        .map::<(Range, Range), _>(|pair| {
            let (sections_1, sections_2) = pair.split_once(",").unwrap();
            (sections_1.into(), sections_2.into())
        })

        // filter for only pairs with overlapping ranges
        .filter(|(section_1, section_2)| {
            section_1.contains(section_2) || section_2.contains(section_1)
        })

        // .inspect(|thing| println!("{:?}", thing))
        
        .count();

    println!("total: {}", count);
    Ok(())
}

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    pub fn contains(&self, other: &Range) -> bool {
        return self.start <= other.start && self.end >= other.end;
    }
}
#[test]
fn range_contains() {
    let r: Range = "5-10".into();
    assert!(r.contains(&"6-8".into()));
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let (start, end) = value.split_once("-").unwrap();
        Range {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        }
    }
}
#[test]
fn range_from_string() {
    let r: Range = "32-55".into();
    assert_eq!(r.start, 32);
    assert_eq!(r.end, 55);

    let r: Range = "3-855".into();
    assert_eq!(r.start, 3);
    assert_eq!(r.end, 855);
}
