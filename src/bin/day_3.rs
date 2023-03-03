#[derive(Clone, Debug)]
struct Rucksack {
    shared_item: char,
}

impl Rucksack {
    fn from(contents: &str) -> Rucksack {
        let compartment_size = contents.len() / 2;

        let first = &contents[0..compartment_size];
        let second = &contents[compartment_size..contents.len()];

        Rucksack {
            shared_item: first.chars().find(|&item| second.contains(item)).unwrap(),
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
