static PRIORITIES: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[derive(Clone, Debug)]
struct Rucksack {
    contents: String,
    shared_item: char,
}

#[derive(Clone, Debug)]
struct RucksackGroup {
    rucksacks: Vec<Rucksack>,
    shared_item: char,
}

impl Rucksack {
    fn from(contents: &str) -> Rucksack {
        let compartment_size = contents.len() / 2;

        let first = &contents[0..compartment_size];
        let second = &contents[compartment_size..contents.len()];

        Rucksack {
            contents: contents.to_string(),
            shared_item: first.chars().find(|&item| second.contains(item)).unwrap(),
        }
    }

    fn shared_item_priority(&self) -> i16 {
        PRIORITIES
            .iter()
            .position(|&c| c == self.shared_item)
            .unwrap() as i16
            + 1
    }
}

impl RucksackGroup {
    fn from(first: &str, second: &str, third: &str) -> RucksackGroup {
        RucksackGroup {
            rucksacks: vec![
                Rucksack::from(first),
                Rucksack::from(second),
                Rucksack::from(third),
            ],
            shared_item: 'a',
        }
    }
}

fn groups_from_input(input: &str) -> Vec<RucksackGroup> {
    let mut lines = input.lines();
    let mut groups: Vec<RucksackGroup> = vec![];

    loop {
        let first = lines.next();

        match first {
            None => break,
            Some(first) => groups.push(RucksackGroup::from(
                first,
                lines.next().unwrap(),
                lines.next().unwrap(),
            )),
        }
    }

    groups
}

fn main() -> anyhow::Result<()> {
    let input: &'static str = include_str!("../../inputs/day_3.txt");

    let rucksacks: Vec<Rucksack> = input.lines().map(Rucksack::from).collect();

    println!(
        "{}",
        rucksacks
            .iter()
            .map(|rucksack| rucksack.shared_item_priority())
            .sum::<i16>()
    );

    let groups = groups_from_input(input);

    println!("{:#?}", groups);

    Ok(())
}
