pub fn find(array: &[i32], key: i32) -> Option<usize> {
    match array.len() {
        0 => None,
        n => binary_search_rec(array, key, 0, n - 1),
    }
}

fn binary_search_rec(array: &[i32], key: i32, left: usize, right: usize) -> Option<usize> {
    if left > right {
        return None; // Base case: range is invalid
    }

    // Calculate mid-point
    let mid = left + (right - left) / 2; // Avoid overflow with (left + right) / 2

    match array[mid].cmp(&key) {
        std::cmp::Ordering::Equal => Some(mid), // Found the key
        std::cmp::Ordering::Less => {
            // Search in the right half
            if mid == array.len() - 1 {
                None // Prevent recursion going out of bounds
            } else {
                binary_search_rec(array, key, mid + 1, right)
            }
        }
        std::cmp::Ordering::Greater => {
            // Search in the left half
            if mid == 0 {
                None // Prevent recursion going out of bounds
            } else {
                binary_search_rec(array, key, left, mid - 1)
            }
        }
    }
}
