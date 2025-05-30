use super::*;

#[test]
fn test_option_array() {
    assert_eq!(
        option_array![1, null, 2, null, null, null, 3],
        [Some(1), None, Some(2), None, None, None, Some(3)]
    );
}

#[test]
fn test_btree() {
    assert_eq!(
        btree![1, null, 2, null, null, null, 3],
        new_tree(1, None, new_tree(2, None, new_tree(3, None, None)))
    );
}
