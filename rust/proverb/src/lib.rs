const EMPTY: &str = "";

pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return EMPTY.to_string();
    }

    let mut proverb = String::new();
    if has_at_least_one_pair(list) {
        proverb.push_str(&create_item_pairs_part(list));
    }

    let first_item = list.first().unwrap_or(&"nail");
    proverb.push_str(&*format!("And all for the want of a {}.", first_item));

    proverb
}

fn has_at_least_one_pair(list: &[&str]) -> bool {
    list.len() >= 2
}

fn create_item_pairs_part(list: &[&str]) -> String {
    list.windows(2)
        .map(|item_pair| match item_pair {
            [first, second] => format!("For want of a {} the {} was lost.\n", first, second),
            _ => "".to_string(),
        })
        .collect()
}
