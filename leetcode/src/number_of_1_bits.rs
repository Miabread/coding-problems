pub fn shifts(mut n: u32) -> i32 {
    let mut count = 0;
    while n != 0 {
        count += n & 1;
        n >>= 1;
    }
    count as i32
}

pub fn string(n: u32) -> i32 {
    format!("{:b}", n).chars().filter(|it| *it == '1').count() as i32
}

#[test]
fn test_shifts() {
    assert_eq!(3, shifts(0b00000000000000000000000000001011));
    assert_eq!(1, shifts(0b00000000000000000000000010000000));
    assert_eq!(31, shifts(0b11111111111111111111111111111101));
}

#[test]
fn test_string() {
    assert_eq!(3, string(0b00000000000000000000000000001011));
    assert_eq!(1, string(0b00000000000000000000000010000000));
    assert_eq!(31, string(0b11111111111111111111111111111101));
}
