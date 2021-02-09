pub fn build_proverb(list: &[&str]) -> String {

    if list.is_empty() {
        return String::new();
    }

    let mut proverb = String::new();
    if has_at_least_one_pair(list) {
        let overlapping_pairs = list.windows(2);
        for item_pair in overlapping_pairs {
            match item_pair {
                [first, second] => proverb.push_str(& *format!("For want of a {} the {} was lost.\n", first, second)),
                _ => panic!("Oh no! :o")
            }
        }
    }

    let first_item = list.first().unwrap();
    proverb.push_str(&*format!("And all for the want of a {}.", first_item));

    proverb
}

fn has_at_least_one_pair(list: &[&str]) -> bool {
    list.len() >= 2
}