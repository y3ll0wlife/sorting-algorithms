#[allow(dead_code)]
pub fn inseration_sort(arr: &mut Vec<i32>) {
    for step in 1..arr.len() {
        let key = arr[step];
        let mut j: i32 = (step - 1).try_into().unwrap();

        while j >= 0 && key < arr[j as usize] {
            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1;
        }

        arr[(j + 1) as usize] = key;
    }
}
