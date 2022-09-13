pub fn quick_sort(arr: &mut Vec<i32>, low: i32, high: i32) {
    if low < high {
        let pi = partition(arr, low, high);

        quick_sort(arr, low, pi - 1);
        quick_sort(arr, pi + 1, high)
    }
}

fn partition(arr: &mut Vec<i32>, low: i32, high: i32) -> i32 {
    let pivot = arr[high as usize];
    let mut i = low - 1;

    for j in low..high {
        if arr[j as usize] <= pivot {
            i += 1;

            let tmp = arr[i as usize];
            arr[i as usize] = arr[j as usize];
            arr[j as usize] = tmp;
        }
    }

    let tmp = arr[(i + 1) as usize];
    arr[(i + 1) as usize] = arr[high as usize];
    arr[high as usize] = tmp;

    return i + 1;
}
