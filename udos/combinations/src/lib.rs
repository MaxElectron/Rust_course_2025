#![forbid(unsafe_code)]

pub fn combinations(arr: &[i32], k: usize) -> Vec<Vec<i32>> {
    // base cases

    if k == 0 {
        return vec![Vec::new()];
    }

    if arr.is_empty() {
        return Vec::new();
    }

    if k > arr.len() {
        return Vec::new();
    }

    // first element base case

    let first = arr[0];
    let rest = combinations(&arr[1..], k - 1);
    let combinations_with_first = rest
        .iter()
        .map(|suffix| {
            let mut combination = Vec::new();
            combination.push(first);
            combination.extend_from_slice(suffix);
            combination
        })
        .collect::<Vec<Vec<i32>>>();

    // recursive case

    let combinations_without_first = combinations(&arr[1..], k);

    // result

    [combinations_with_first, combinations_without_first].concat()
}
