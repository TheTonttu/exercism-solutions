use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mid_index = match array.len() {
        0 | 1 => 0,
        2..=usize::MAX => (array.len() - 1) / 2,
        _ => panic!("wat"),
    };

    if let Some(mid_value) = array.get(mid_index) {
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
    } else {
        None
    }
}
