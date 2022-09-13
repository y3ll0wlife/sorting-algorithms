#[allow(dead_code)]
pub fn bubble_sort(arr: &mut Vec<i32>) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                let tmp = arr[j];

                arr[j] = arr[j + 1];
                arr[j + 1] = tmp;
            }
        }
    }
}

#[allow(dead_code)]
pub fn bubble_sort_optimzed(arr: &mut Vec<i32>) {
    for i in 0..arr.len() {
        let mut swapped = false;

        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                let tmp = arr[j];

                arr[j] = arr[j + 1];
                arr[j + 1] = tmp;

                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}
