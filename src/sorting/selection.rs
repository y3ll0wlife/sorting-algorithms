pub fn selection_sort(arr: &mut Vec<i32>) {
    let size = arr.len();

    for step in 0..size {
        let mut min_index = step;

        for i in step + 1..size {
            if arr[i] < arr[min_index] {
                min_index = i;
            }
        }

        let tmp = arr[step];
        arr[step] = arr[min_index];
        arr[min_index] = tmp;
    }
}
