pub fn merge_sort(arr: &mut Vec<i32>, l: i32, r: i32) {
    if l < r {
        let m = l + (r - l) / 2;

        merge_sort(&mut arr, l, m);
        merge_sort(&arr, m + 1, r);

        merge(arr, l, m, r);
    }
}

fn merge(arr: &mut Vec<i32>, p: i32, q: i32, r: i32) {
    let n1: i32 = q - p + 1;
    let n2: i32 = r - q;

    let mut a1 = vec![0; n1 as usize];
    let mut a2 = vec![0; n2 as usize];

    for i in 0..n1 {
        a1[i as usize] = arr[(p + i) as usize];
    }

    for j in 0..n2 {
        a2[j as usize] = arr[(q + 1 + j) as usize];
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = p;

    while i < n1 && j < n2 {
        if a1[i as usize] <= a2[j as usize] {
            arr[k as usize] = a1[i as usize];
            i += 1;
        } else {
            arr[k as usize] = a2[j as usize];
            j += 1;
        }
    }

    while i < n1 {
        arr[k as usize] = a1[i as usize];
        i += 1;
        k += 1;
    }

    while j < n2 {
        arr[k as usize] = a2[j as usize];
        j += 1;
        k += 1;
    }
}
