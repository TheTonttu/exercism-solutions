use std::cmp::Ordering::{Equal, Greater, Less};

pub fn find<T: Ord, U: AsRef<[T]>>(array: U, key: T) -> Option<usize> {
    let values = array.as_ref();
    let mid_index = values.len() / 2;

    match key.cmp(values.get(mid_index)?) {
        Less => find(&values[..mid_index], key),
        Equal => Some(mid_index),
        Greater => find(&values[mid_index + 1..], key).map(|i| i + mid_index + 1),
    }
}
