#[derive(Copy, Clone, Debug, PartialEq)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl TryFrom<&char> for Hand {
    type Error = &'static str;

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Hand::Rock),
            'B' | 'Y' => Ok(Hand::Paper),
            'C' | 'Z' => Ok(Hand::Scissors),
            _ => Err("Could not figure out hand from character!"),
        }
    }
}

/// calculate your score for a round of rock-paper-scissors
fn calculate_round_score(opponent: Hand, you: Hand) -> i32 {
    match (opponent, you) {
        // tie
        (Hand::Rock, Hand::Rock)
        | (Hand::Paper, Hand::Paper)
        | (Hand::Scissors, Hand::Scissors) => 3 + you as i32,
        // win
        (Hand::Rock, Hand::Paper)
        | (Hand::Paper, Hand::Scissors)
        | (Hand::Scissors, Hand::Rock) => 6 + you as i32,
        // lose
        (Hand::Rock, Hand::Scissors)
        | (Hand::Paper, Hand::Rock)
        | (Hand::Scissors, Hand::Paper) => you as i32,
    }
}

fn part_one(input: &str) -> anyhow::Result<()> {
    let total_score: i32 = input
        .lines()
        .map(|s| s.chars().collect())
        .map(|c: Vec<char>| {
            vec![
                Hand::try_from(&c[0]).unwrap(),
                Hand::try_from(&c[2]).unwrap(),
            ]
        })
        .map(|r| calculate_round_score(r[0], r[1]))
        .sum();

    println!("part one: total score: {}", total_score);
    Ok(())
}

fn part_two(input: &str) -> anyhow::Result<()> {
    let total_score = 0;

    println!("part 2: total score: {}", total_score);
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let input: &'static str = include_str!("../../inputs/day_2.txt");

    part_one(input)?;
    part_two(input)?;
    Ok(())
}
