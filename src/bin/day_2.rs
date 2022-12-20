#[derive(Debug, PartialEq)]
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

// calculate your score for a round of rock-paper-scissors
fn calculate_round_score(opponent: Hand, you: Hand) -> i32 {
    // draw
    if you == opponent {
        return 3 + you as i32;
    }

    // lose
    if (you == Hand::Rock && opponent == Hand::Paper)
        || (you == Hand::Paper && opponent == Hand::Scissors)
        || (you == Hand::Scissors && opponent == Hand::Rock)
    {
        return you as i32;
    }

    // win
    if (you == Hand::Rock && opponent == Hand::Scissors)
        || (you == Hand::Paper && opponent == Hand::Rock)
        || (you == Hand::Scissors && opponent == Hand::Paper)
    {
        return 6 + you as i32;
    }

    unreachable!();
}

fn main() -> anyhow::Result<()> {
    let rounds: Vec<Vec<char>> = include_str!("../../inputs/day_2.txt")
        .lines()
        .map(|s| s.chars().collect())
        .collect();

    let mut total_score = 0;

    for round in rounds {
        let opponent = Hand::try_from(round.first().unwrap()).unwrap();
        let you = Hand::try_from(round.last().unwrap()).unwrap();

        total_score += calculate_round_score(opponent, you);
    }

    println!("total score: {}", total_score);

    Ok(())
}
