pub fn valid_palindrome(input: &str) -> bool {
    let input = input.to_ascii_lowercase();
    let mut input = input.chars().filter(|it| it.is_ascii_alphanumeric());

    while let (Some(front), Some(back)) = (input.next(), input.next_back()) {
        if front != back {
            return false;
        }
    }

    true
}

#[test]
fn test_valid_palindrome() {
    assert_eq!(true, valid_palindrome("A man, a plan, a canal: Panama"));
    assert_eq!(false, valid_palindrome("race a car"));
}
