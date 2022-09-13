mod sorting;

fn main() {
    let mut arr: Vec<i32> = vec![9, 5, 4, 2, 10, 5, 50, 1, 5, -10, -5];

    sorting::selection::selection_sort(&mut arr);

    println!("{:?}", arr);
}
