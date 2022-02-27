#![allow(dead_code)]
fn median(nums: Vec<f32>) -> Option<f32> {
    let mut nums = nums.clone();
    nums.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let len = nums.len();
    match len {
        0 => None,
        _ if len % 2 == 0 => {
            if let [left, right] = nums[len / 2 - 1..len / 2 + 1] {
                Some((left + right) / 2.0)
            } else {
                None
            }
        }
        _ => Some(nums[(len - 1) / 2]),
    }
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}
