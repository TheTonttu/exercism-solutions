// Hölökyn kölökyn
const MAX_BEER_COUNT:u32 = 99;

pub fn verse(n: u32) -> String {
    match n {
        // multiple beers
        n if n >= 3 => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1),
        // two beers, transition from plural to singular
        n if n == 2 => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", n, n, n-1),
        // one beer
        n if n == 1 => format!("{} bottle of beer on the wall, {} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n", n, n),
        // zero beers
        n if n == 0 => format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, {} bottles of beer on the wall.\n", MAX_BEER_COUNT),
        _ => panic!("Deal with it.")
    }
}

pub fn sing(start: u32, end: u32) -> String {
    assert!(start >= end);

    let verse_count = (start-end)+1;
    let mut verses = Vec::with_capacity(verse_count as usize);
    for n in (end..=start).rev() {
        verses.push(verse(n));
    }
    let lyrics = verses.join("\n");

    lyrics
}
