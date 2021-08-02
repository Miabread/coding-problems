pub fn bits(mut n: i32) -> bool {
    let mut seen = std::collections::HashSet::new();

    while n != 1 {
        let mut total = 0;

        while n > 0 {
            let digit = n % 10;
            total += digit * digit;
            n /= 10;
            dbg!(&total);
        }

        if seen.contains(&total) {
            return false;
        }

        seen.insert(total);
        n = total;
    }

    true
}

pub fn string(mut n: i32) -> bool {
    let mut seen = std::collections::HashSet::new();

    while n != 1 {
        n = n
            .to_string()
            .chars()
            .map(|char| char.to_digit(10).unwrap().pow(2) as i32)
            .sum();

        if seen.contains(&n) {
            return false;
        }

        seen.insert(n);
    }

    true
}

#[test]
fn test_bits() {
    assert_eq!(true, bits(19));
    assert_eq!(false, bits(2));
}

#[test]
fn test_string() {
    assert_eq!(true, string(19));
    assert_eq!(false, string(2));
}
