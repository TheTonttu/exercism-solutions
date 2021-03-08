use std::cmp::Ordering;

pub fn find<K: Ord, A: AsRef<[K]>>(array: A, key: K) -> Option<usize> {
    let values = array.as_ref();

    let mid_index = values.len() / 2;

    match key.cmp(values.get(mid_index)?) {
        Ordering::Less => find(&values[0..mid_index], key),
        Ordering::Equal => Some(mid_index),
        Ordering::Greater => find(&values[mid_index + 1..], key).map(|i| i + mid_index + 1),
    }
}
