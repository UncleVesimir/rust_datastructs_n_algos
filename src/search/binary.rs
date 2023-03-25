// O(log(N))

pub fn binary_search(arr: &[usize], needle: usize) -> bool {
    let mut lo = 0;
    let mut high = arr.len(); //overflow here.

    while lo < high {
        let mid: usize = lo + (high - lo) / 2;
        let value = arr[mid];
        if value == needle {
            return true;
        } else if needle > value {
            lo = mid + 1;
        } else {
            high = mid;
        }
    }
    false
}

#[cfg(test)]

mod tests {
    use super::binary_search;

    #[test]
    fn binary_search_works() {
        let arr: [usize; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let arr_empty: [usize; 0] = [];
        let needle = 4;

        assert!(binary_search(&arr, needle));
        assert_eq!(binary_search(&arr, 11), false);
        assert_eq!(binary_search(&arr_empty, 4), false);
    }
}
