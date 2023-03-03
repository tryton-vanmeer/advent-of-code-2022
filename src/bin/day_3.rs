#[derive(Clone, Debug)]
struct Rucksack {
    fist_compartment: String,
    second_compartment: String,
}

impl Rucksack {
    fn from(contents: &str) -> Rucksack {
        let compartment_size = contents.len() / 2;

        Rucksack {
            fist_compartment: contents[0..compartment_size].to_string(),
            second_compartment: contents[compartment_size..contents.len()].to_string(),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let input: &'static str = include_str!("../../inputs/day_3.txt");
    let mut rucksacks: Vec<Rucksack> = vec![];

    for line in input.lines() {
        rucksacks.push(Rucksack::from(line));
    }

    println!("{:#?}", rucksacks);

    Ok(())
}
