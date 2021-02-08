// Hölökyn kölökyn
const MAX_BEER_COUNT: u32 = 99;

pub fn verse(n: u32) -> String {
    match n {
        // multiple beers
        bottles if bottles >= 3 => format!("{beers} bottles of beer on the wall, {beers} bottles of beer.\nTake one down and pass it around, {beers_left} bottles of beer on the wall.\n", beers = bottles, beers_left = bottles - 1),
        // two beers, transition from plural to singular
        bottles if bottles == 2 => format!("{beers} bottles of beer on the wall, {beers} bottles of beer.\nTake one down and pass it around, {beers_left} bottle of beer on the wall.\n", beers = bottles, beers_left = bottles - 1),
        // one beer --> zero beers :(
        bottles if bottles == 1 => format!("{beers} bottle of beer on the wall, {beers} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n", beers = bottles),
        // zero beers :( --> more beer :)
        bottles if bottles == 0 => format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, {all_the_beer} bottles of beer on the wall.\n", all_the_beer = MAX_BEER_COUNT),
        // (•_•) ( •_•)>⌐□-□ (⌐□_□)
        _ => panic!("Deal with it.")
    }
}

pub fn sing(start: u32, end: u32) -> String {
    assert!(start <= MAX_BEER_COUNT, "Take it easy! (99 beers is max)");
    assert!(
        start >= end,
        "We should consume it. (start should be >= end)"
    );

    let verse_count = (start - end) + 1;
    let mut verses = Vec::with_capacity(verse_count as usize);
    for n in (end..=start).rev() {
        verses.push(verse(n));
    }
    verses.join("\n")
}
