use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool {
    if _first_list.len() == 0 {
        return true;
    }
    for list in _second_list.windows(_first_list.len()) {
        if _first_list == list {
            return true;
        }
    }
    return false;
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    // unimplemented!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");
    if _first_list == _second_list {
        return Comparison::Equal;
    }
    if is_sublist(&_first_list, &_second_list) {
        return Comparison::Sublist;
    }
    if is_sublist(&_second_list, &_first_list) {
        return Comparison::Superlist;
    }

    Comparison::Unequal
}
