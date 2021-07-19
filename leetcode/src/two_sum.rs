pub fn double_loop(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, x) in numbers.iter().enumerate() {
        for (j, y) in numbers.iter().enumerate() {
            if i != j && x + y == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    unreachable!();
}

pub fn pre_map(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let entries: HashMap<_, _> = numbers.iter().enumerate().map(|(i, x)| (x, i)).collect();

    for (i, x) in numbers.iter().enumerate() {
        let y = target - x;

        if let Some(&j) = entries.get(&y) {
            if i != j {
                return vec![i as i32, j as i32];
            }
        }
    }

    unreachable!()
}

#[test]
fn double_loop_test() {
    assert_eq!(vec![0, 1], double_loop(vec![2, 7, 11, 15], 9));
    assert_eq!(vec![1, 2], double_loop(vec![3, 2, 4], 6));
    assert_eq!(vec![0, 1], double_loop(vec![3, 3], 6));
}

#[test]
fn pre_map_test() {
    assert_eq!(vec![0, 1], pre_map(vec![2, 7, 11, 15], 9));
    assert_eq!(vec![1, 2], pre_map(vec![3, 2, 4], 6));
    assert_eq!(vec![0, 1], pre_map(vec![3, 3], 6));
}
