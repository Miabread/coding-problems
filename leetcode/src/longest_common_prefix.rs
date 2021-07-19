pub fn longest_common_prefix(strings: Vec<String>) -> String {
    strings
        .iter()
        .fold(None, |last: Option<&str>, next| {
            Some(
                last.map(|last| {
                    last.char_indices()
                        .zip(next.char_indices())
                        .take_while(|(x, y)| x.1 == y.1)
                        .last()
                        .map(|(x, _)| &last[0..=x.0])
                        .unwrap_or_default()
                })
                .unwrap_or(next),
            )
        })
        .unwrap()
        .to_owned()
}

#[test]
fn longest_common_prefix_test() {
    assert_eq!(
        "fl".to_owned(),
        longest_common_prefix(vec![
            "flower".to_owned(),
            "flow".to_owned(),
            "flight".to_owned()
        ])
    );
    assert_eq!(
        "",
        longest_common_prefix(vec![
            "dog".to_owned(),
            "racecar".to_owned(),
            "car".to_owned()
        ])
    );
}
