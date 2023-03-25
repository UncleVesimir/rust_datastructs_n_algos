// N (N + 1)/2 -> O(N^2)

pub fn bubble_sort(arr: &mut [usize]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j + 1] > arr[j] {
                let tmp = arr[j];
                arr[j + 1] = arr[j];
                arr[j] = tmp
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::bubble_sort;

    fn bubble_sort_works() {
        let mut arr: [usize; 9] = [1, 2, 6, 9, 3, 1, 3, 4, 8];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 1, 2, 3, 3, 4, 6, 8, 9]);
    }
}
