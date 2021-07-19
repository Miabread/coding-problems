pub fn remove_element(nums: &mut Vec<i32>, value: i32) -> i32 {
    let mut write_index = 0;

    for read_index in 0..nums.len() {
        if nums[read_index] == value {
            continue;
        }

        nums[write_index] = nums[read_index];
        write_index += 1;
    }

    write_index as i32
}
