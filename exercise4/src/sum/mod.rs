pub fn get_sum_of_array(arr: &[u32]) -> Option<u32> {
    if arr.is_empty() {
        return None;
    }
    let mut sum: u32 = 0;
    for i in arr {
        sum = sum.checked_add(*i)?;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        assert_eq!(get_sum_of_array(&[]), None);
    }
    #[test]
    fn normal() {
        let numbers = [4u32, 3u32, 2u32, 1u32];
        assert_eq!(get_sum_of_array(numbers.as_slice()), Some(10));
    }
    #[test]
    fn overflow() {
        let x = [1, u32::MAX];
        assert_eq!(get_sum_of_array(&x), None);
    }
}
