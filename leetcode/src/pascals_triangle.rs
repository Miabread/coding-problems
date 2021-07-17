pub fn pascals_triangle(num_rows: i32) -> Vec<Vec<i32>> {
    use std::convert::TryInto;
    let num_rows = num_rows.try_into().unwrap();

    let mut rows: Vec<Vec<i32>> = Vec::with_capacity(num_rows);

    for i in 0..num_rows {
        rows.push(
            (0..=i)
                .map(|j| {
                    if j == 0 || j == i {
                        1
                    } else {
                        rows[i - 1][j - 1] + rows[i - 1][j]
                    }
                })
                .collect(),
        );
    }

    rows
}

#[test]
fn test_pascals_triangle() {
    assert_eq!(
        pascals_triangle(5),
        vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1]
        ]
    );
    assert_eq!(pascals_triangle(1), vec![vec![1]]);
}
