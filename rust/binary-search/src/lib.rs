use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mid_index = match array.len() {
        len if len >= 3 => (len - 1) / 2,
        _ => 0,
    };

    match array.get(mid_index) {
        Some(mid_value) => {
            match mid_value.cmp(&key) {
                Ordering::Less => match find(&array[mid_index + 1..], key) {
                    Some(sub_index) => Some((mid_index + 1) + sub_index),
                    None => None,
                },
                Ordering::Equal => Some(mid_index),
                Ordering::Greater => match find(&array[0..mid_index], key) {
                    Some(sub_index) => Some(sub_index),
                    None => None,
                },
            }
        }
        None => None
    }
}
