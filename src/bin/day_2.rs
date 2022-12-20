#[derive(Debug, PartialEq)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn decode_hand(letter: &str) -> Option<Hand> {
    match letter {
        "A" => Some(Hand::Rock),
        "B" => Some(Hand::Paper),
        "C" => Some(Hand::Scissors),
        "X" => Some(Hand::Rock),
        "Y" => Some(Hand::Paper),
        "Z" => Some(Hand::Scissors),
        _ => None,
    }
}

// calculate your score for a round of rock-paper-scissors
fn calculate_round_score(opponent: Hand, you: Hand) -> u32 {
    let mut score = 0;

    // points for hand selected
    match you {
        Hand::Rock => score += 1,
        Hand::Paper => score += 2,
        Hand::Scissors => score += 3,
    }

    // draw
    if you == opponent {
        score += 3
    }

    // lose
    if (you == Hand::Rock && opponent == Hand::Paper)
        || (you == Hand::Paper && opponent == Hand::Scissors)
        || (you == Hand::Scissors && opponent == Hand::Rock)
    {
        score += 0
    }

    // win
    if (you == Hand::Rock && opponent == Hand::Scissors)
        || (you == Hand::Paper && opponent == Hand::Rock)
        || (you == Hand::Scissors && opponent == Hand::Paper)
    {
        score += 6
    }

    score
}

fn main() -> anyhow::Result<()> {
    let lines: Vec<&str> = include_str!("../../inputs/day_2.txt").lines().collect();
    let mut total_score = 0;

    for line in lines {
        let opponent = decode_hand(line.get(0..1).unwrap()).unwrap();
        let you = decode_hand(line.get(2..3).unwrap()).unwrap();

        total_score += calculate_round_score(opponent, you);
    }

    println!("total score: {}", total_score);

    Ok(())
}
