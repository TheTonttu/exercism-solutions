use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut transformed_data = BTreeMap::new();
    for (points, chars) in h.iter() {
        transformed_data.extend(chars.iter().map(|c| (c.to_ascii_lowercase(), *points)))
    }

    transformed_data
}
