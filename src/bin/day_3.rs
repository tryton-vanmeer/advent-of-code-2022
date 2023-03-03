fn main() -> anyhow::Result<()> {
    let input: &'static str = include_str!("../../inputs/day_3.txt");

    for line in input.lines() {
        println!("{}", line);
    }

    Ok(())
}
