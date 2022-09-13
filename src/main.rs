use std::convert::TryInto;

mod sorting;

fn main() {
    let mut arr: Vec<i32> = vec![9, 5, 4, 2, 10, 5, 50, 1, 5, -10, -5];
    sorting::merge::merge_sort(&mut arr, 0, (arr.len() - 1).try_into().unwrap());

    println!("{:?}", arr);
}
