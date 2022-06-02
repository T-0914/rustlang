use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}
pub fn match_all<T: PartialEq + Debug>(_first_list: &Vec<T>, _second_list: &Vec<T>) -> bool {
    println!("{:?} - {:?}", _first_list, _second_list);
    _first_list
        .iter()
        .zip(_second_list.iter())
        .all(|(a, b)| a == b)
}

// impl Foo for Slice {}

pub fn slice<T: Debug + Copy>(index: usize, len: usize, _list: &Vec<T>) -> Vec<T> {
    let mut vec = Vec::new();
    for index in index..=len + index - 1 {
        vec.push(_list[index]);
    }
    return vec;
}

pub fn sublist<T: PartialEq + Clone + Debug + Copy>(
    _first_list: &[T],
    _second_list: &[T],
) -> Comparison {
    // unimplemented!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");
    if _first_list.len() > 0 && _second_list.len() == 0 {
        return Comparison::Superlist;
    }
    if _first_list.len() == 0 && _second_list.len() > 0 {
        return Comparison::Sublist;
    }
    if _first_list.len() != _second_list.len() {
        if _first_list.len() < _second_list.len() {
            let length: usize = _second_list.len() - _first_list.len();
            let mut flag = false;

            for i in 0..=length {
                let _slice = slice(i, _first_list.len(), &_second_list.to_vec());
                let is_match_all = match_all(&_first_list.to_vec(), &_slice.to_vec());

                if is_match_all == true {
                    flag = true;
                    break;
                }
            }
            return match flag {
                true => Comparison::Sublist,
                false => Comparison::Unequal,
            };
        } else {
            let length: usize = _first_list.len() - _second_list.len();
            let mut flag = false;

            for i in 0..=length {
                let _slice = slice(i, _second_list.len(), &_first_list.to_vec());
                let is_match_all = match_all(&_second_list.to_vec(), &_slice.to_vec());

                if is_match_all == true {
                    flag = true;
                    break;
                }
            }
            return match flag {
                true => Comparison::Superlist,
                false => Comparison::Unequal,
            };
        }
    } else {
        let is_match_all = match_all(&_first_list.to_vec(), &_second_list.to_vec());

        match is_match_all {
            true => Comparison::Equal,
            false => Comparison::Unequal,
        }
    }
}
