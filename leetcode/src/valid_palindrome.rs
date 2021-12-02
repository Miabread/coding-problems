pub fn iterator(input: &str) -> bool {
    let mut input = input
        .chars()
        .filter(|it| it.is_ascii_alphanumeric())
        .map(|it| it.to_ascii_lowercase());

    while let (Some(front), Some(back)) = (input.next(), input.next_back()) {
        if front != back {
            return false;
        }
    }

    true
}

pub fn match_loop(input: &str) -> bool {
    let mut input: &[_] = &input
        .chars()
        .filter(|it| it.is_ascii_alphanumeric())
        .map(|it| it.to_ascii_lowercase())
        .collect::<Vec<_>>();

    loop {
        match input {
            // Exactly zero or one items left
            [] | [_] => return true,
            // If first and last items match, continue to check the middle
            [first, middle @ .., last] if first == last => input = middle,
            // Anything else isn't valid
            [..] => return false,
        }
    }
}

#[test]
fn test_iterator() {
    assert!(iterator("A man, a plan, a canal: Panama"));
    assert!(!iterator("race a car"));
}

#[test]
fn test_match_loop() {
    assert!(match_loop("A man, a plan, a canal: Panama"));
    assert!(!match_loop("race a car"));
}
