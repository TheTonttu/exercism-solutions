use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mid = (array.len() - 1) / 2;
    if let Some(mid_value) = array.get(mid) {
        match mid_value.cmp(&key) {
            Ordering::Less => None, /* TODO: Search right */
            Ordering::Equal => Some(mid),
            Ordering::Greater => None, /* TODO: Search left */
        }
    } else {
        None
    }
}
