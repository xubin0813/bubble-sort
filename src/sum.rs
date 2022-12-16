pub fn sum(list: &[u32]) -> Option<u32> {
    let mut result: Option<u32> = Some(0);
    for i in 0..list.len() {
        result = result.unwrap().checked_add(list[i]);
    }
    return result;
}

#[test]
fn test_normal() {
    let list = [9, 8, 7, 6, 5];
    let result = sum(&list);
    assert_eq!(result, Some(35));
}

#[test]
fn test_overflow() {
  let list = [u32::MAX, 9];
  let result = sum(&list);
  assert_eq!(result, None);
}
