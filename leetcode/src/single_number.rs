pub fn single_number_set(numbers: &[i32]) -> i32 {
    let mut seen = std::collections::HashSet::new();

    for number in numbers {
        if seen.contains(number) {
            seen.remove(number);
        } else {
            seen.insert(number);
        }
    }

    *seen.into_iter().next().unwrap()
}

pub fn single_number_bits(numbers: &[i32]) -> i32 {
    numbers.iter().fold(0, std::ops::BitXor::bitxor)
}

#[test]
fn test_single_number_set() {
    assert_eq!(1, single_number_set(&[2, 2, 1]));
    assert_eq!(4, single_number_set(&[4, 1, 2, 1, 2]));
    assert_eq!(1, single_number_set(&[1]));
}

#[test]
fn test_single_number_bits() {
    assert_eq!(1, single_number_bits(&[2, 2, 1]));
    assert_eq!(4, single_number_bits(&[4, 1, 2, 1, 2]));
    assert_eq!(1, single_number_bits(&[1]));
}
