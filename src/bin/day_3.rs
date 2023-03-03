#[derive(Clone, Debug)]
struct Rucksack {
    // fist_compartment: String,
    // second_compartment: String,
    shared_item: char,
}

fn find_shared_item(first: &str, second: &str) -> Option<char> {
    for item in first.chars() {
        if second.contains(item) {
            return Some(item)
        }
    }

    None
}

impl Rucksack {
    fn from(contents: &str) -> Rucksack {
        let compartment_size = contents.len() / 2;

        let first = &contents[0..compartment_size];
        let second = &contents[compartment_size..contents.len()];

        Rucksack {
            // fist_compartment: first.to_string(),
            // second_compartment: second.to_string(),
            shared_item: find_shared_item(first, second).unwrap()
        }
    }
}

fn main() -> anyhow::Result<()> {
    let input: &'static str = include_str!("../../inputs/day_3.txt");
    let mut rucksacks: Vec<Rucksack> = vec![];

    for line in input.lines() {
        rucksacks.push(Rucksack::from(line));
    }

    // println!("{:#?}", rucksacks);

    for rucksack in rucksacks {
        println!("{}", rucksack.shared_item);
    }

    Ok(())
}
