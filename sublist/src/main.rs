use sublist::{match_all, slice, sublist};

pub fn main() {
    let first_list = vec![1, 2, 3];
    let second_list = vec![1, 3];
    println!("{:?}", sublist(&first_list, &second_list));
}
