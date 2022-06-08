#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list.len() == second_list.len() && first_list == second_list {
        Comparison::Equal
    } else if first_list.len() < second_list.len() && is_sublist(first_list, second_list) {
        Comparison::Sublist
    } else if first_list.len() > second_list.len() && is_sublist(second_list, first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

fn is_sublist<T: PartialEq>(sublist: &[T], superlist: &[T]) -> bool {
    if sublist.is_empty() {
        return true;
    }

    if let Some(expected_start_value) = sublist.first() {
        for (start_index, start_value) in superlist.iter().enumerate() {
            let remaining_length = superlist.len() - start_index;
            if remaining_length < sublist.len() {
                return false;
            }
            if start_value == expected_start_value {
                let stop_index = start_index + sublist.len();
                let candidate_slice = &superlist[start_index..stop_index];
                if sublist == candidate_slice {
                    return true;
                }
            }
        }
    }
    false
}
