// O(N)
pub fn linear_search(arr: &Vec<&str>, needle: &str) -> bool {
    for &elem in arr.iter() {
        if needle == elem {
            return true;
        }
    }
    false
}

#[cfg(test)]

mod tests {
    use super::linear_search;

    #[test]
    fn linear_search_works() {
        let arr = vec!["hello", "my", "name", "is", "jonny"];
        let needle = "jonny";

        assert!(linear_search(&arr, needle));
        assert_eq!(linear_search(&arr, "wrong"), false);
    }
}
