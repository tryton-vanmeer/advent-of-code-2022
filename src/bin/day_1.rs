use advent_of_code_2022::Args;

use clap::Parser;

fn get_elves(filename: String) -> anyhow::Result<Vec<i32>> {
    let mut elf_calories: Vec<i32> = Vec::new();
    let mut foods: Vec<i32> = Vec::new();

    if let Ok(lines) = std::fs::read_to_string(filename) {
        for line in lines.split('\n') {
            if line.is_empty() {
                elf_calories.push(foods.iter().sum());
                foods.clear();
            } else {
                foods.push(line.parse::<i32>()?);
            }
        }
    }

    elf_calories.sort_unstable();
    Ok(elf_calories)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut elves = get_elves(args.input)?;

    println!("elf carrying most calories: {}", elves.last().unwrap());

    let top_three = vec![
        elves.pop().unwrap(),
        elves.pop().unwrap(),
        elves.pop().unwrap(),
    ];

    println!(
        "top three elves carrying the most calories: {}",
        top_three.iter().sum::<i32>()
    );

    Ok(())
}
