pub fn raindrops(n: u32) -> String {
    let mut raindrop_sounds = String::with_capacity(3);

    if should_pling(n) {
        raindrop_sounds.push_str("Pling");
    }
    if should_plang(n) {
        raindrop_sounds.push_str("Plang");
    }
    if should_plong(n) {
        raindrop_sounds.push_str("Plong");
    }

    if no_sound(&raindrop_sounds) {
        return n.to_string();
    }

    raindrop_sounds
}

fn should_pling(n: u32) -> bool {
    n % 3 == 0
}

fn should_plang(n: u32) -> bool {
    n % 5 == 0
}

fn should_plong(n: u32) -> bool {
    n % 7 == 0
}

fn no_sound(raindrop: &str) -> bool {
    raindrop.is_empty()
}
