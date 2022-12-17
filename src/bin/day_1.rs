use advent_of_code_2022::get_input_file;

fn get_elves() -> anyhow::Result<Vec<i32>> {
    let mut elf_calories: Vec<i32> = Vec::new();
    let mut foods: Vec<i32> = Vec::new();

    for line in get_input_file()? {
        if line.is_empty() {
            elf_calories.push(foods.iter().sum());
            foods.clear();
        } else {
            foods.push(line.parse::<i32>()?);
        }
    }

    elf_calories.sort_unstable();
    Ok(elf_calories)
}

fn main() -> anyhow::Result<()> {
    let mut elves = get_elves()?;

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
