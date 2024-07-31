use std::collections::HashMap;

#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

pub fn score(dice: Dice, category: Category) -> u8 {
    match category {
        Category::Ones => score_dice_rolls(&dice, 1),
        Category::Twos => score_dice_rolls(&dice, 2),
        Category::Threes => score_dice_rolls(&dice, 3),
        Category::Fours => score_dice_rolls(&dice, 4),
        Category::Fives => score_dice_rolls(&dice, 5),
        Category::Sixes => score_dice_rolls(&dice, 6),
        Category::FullHouse => score_full_house(&dice),
        Category::FourOfAKind => score_four_of_a_kind(&dice),
        Category::LittleStraight => score_little_straight(&dice),
        Category::BigStraight => score_big_straight(&dice),
        Category::Choice => score_choice(&dice),
        Category::Yacht => score_yacht(&dice),
    }
}

fn score_dice_rolls(dice: &Dice, counted_die: u8) -> u8 {
    dice.iter().filter(|&&d| d == counted_die).sum()
}

fn score_full_house(dice: &Dice) -> u8 {
    let expected_counts = [2, 3];
    let full_house_rolls: HashMap<u8, u8> = count_rolls(dice)
        .iter()
        .filter(|(_, &rolls)| (2..=3).contains(&rolls))
        // Note the key <-> value switch
        .map(|(&die, &rolls)| (rolls, die))
        .collect();

    let mut score = 0;
    for &expected_roll in expected_counts.iter() {
        score += match full_house_rolls.get(&expected_roll) {
            Some(die) => die * expected_roll,
            _ => return 0,
        }
    }
    score
}

fn score_four_of_a_kind(dice: &Dice) -> u8 {
    match count_rolls(dice).iter().find(|(_, &count)| count >= 4) {
        Some((&die, _)) => die * 4,
        _ => 0,
    }
}

fn score_little_straight(dice: &Dice) -> u8 {
    let little_straight_rolls: HashMap<u8, u8> = count_rolls(dice)
        .iter()
        .filter(|(&die, _)| (1..=5).contains(&die))
        .map(|(&die, &rolls)| (die, rolls))
        .collect();

    if little_straight_rolls.keys().count() == 5
        && little_straight_rolls.iter().all(|(_, &rolls)| rolls == 1)
    {
        30
    } else {
        0
    }
}

fn score_big_straight(dice: &Dice) -> u8 {
    let big_straight_rolls = count_rolls(dice)
        .iter()
        .filter(|(&die, _)| (2..=6).contains(&die))
        .map(|(&die, &rolls)| (die, rolls))
        .collect::<HashMap<u8, u8>>();

    if big_straight_rolls.keys().count() == 5
        && big_straight_rolls.iter().all(|(_, &rolls)| rolls == 1)
    {
        30
    } else {
        0
    }
}

fn score_choice(dice: &Dice) -> u8 {
    dice.iter().sum()
}

fn score_yacht(dice: &Dice) -> u8 {
    let yacht_rolls = count_rolls(dice);
    match yacht_rolls.iter().next() {
        Some((_, &5)) => 50,
        _ => 0,
    }
}

fn count_rolls(dice: &Dice) -> HashMap<u8, u8> {
    dice.iter().fold(HashMap::new(), |mut rolls, &die| {
        let counter = rolls.entry(die).or_insert(0);
        *counter += 1;
        rolls
    })
}
