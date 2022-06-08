#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() == _second_list.len() && _first_list == _second_list {
        Comparison::Equal
    } else if _first_list.len() < _second_list.len()
        && (_first_list.is_empty() || is_sublist(_first_list, _second_list))
    {
        Comparison::Sublist
    } else if _first_list.len() > _second_list.len()
        && (_second_list.is_empty() || is_sublist(_second_list, _first_list))
    {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

fn is_sublist<T: PartialEq>(sublist: &[T], superlist: &[T]) -> bool {
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
