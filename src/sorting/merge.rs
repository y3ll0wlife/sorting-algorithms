pub fn merge_sort(arr: &mut Vec<i32>, begin: i32, end: i32) {
    if begin >= end {
        return;
    }

    let mid = begin + (end - begin) / 2;

    merge_sort(arr, begin, mid);
    merge_sort(arr, mid + 1, end);

    merge(arr, begin, mid, end);
}

fn merge(arr: &mut Vec<i32>, left: i32, mid: i32, right: i32) {
    let n1: i32 = mid - left + 1;
    let n2: i32 = right - mid;

    let mut left_array = vec![0; n1 as usize];
    let mut right_array = vec![0; n2 as usize];

    for i in 0..n1 {
        left_array[i as usize] = arr[(left + i) as usize];
    }

    for j in 0..n2 {
        right_array[j as usize] = arr[(mid + 1 + j) as usize];
    }

    let mut index_one = 0;
    let mut index_two = 0;
    let mut index_merged = left;

    while index_one < n1 && index_two < n2 {
        if left_array[index_one as usize] <= right_array[index_two as usize] {
            arr[index_merged as usize] = left_array[index_one as usize];
            index_one += 1;
        } else {
            arr[index_merged as usize] = right_array[index_two as usize];
            index_two += 1;
        }
        index_merged += 1;
    }

    while index_one < n1 {
        arr[index_merged as usize] = left_array[index_one as usize];

        index_one += 1;
        index_merged += 1;
    }

    while index_two < n2 {
        arr[index_merged as usize] = right_array[index_two as usize];

        index_two += 1;
        index_merged += 1;
    }
}
