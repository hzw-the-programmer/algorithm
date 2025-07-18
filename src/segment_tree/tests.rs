use super::SegmentTree;

#[test]
fn test_query() {
    let arr = [1, 3, 5, 7, 9];
    let st = SegmentTree::new(&arr);
    assert_eq!(st.query(0, 4), 1 + 3 + 5 + 7 + 9);
    assert_eq!(st.query(1, 3), 3 + 5 + 7);
    assert_eq!(st.query(2, 2), 5);
    assert_eq!(st.query(3, 1), 0);
}

#[test]
fn test_update() {
    let mut st = SegmentTree::new(&[1, 2, 3, 4]);
    assert_eq!(st.query(0, 3), 1 + 2 + 3 + 4);

    st.update(1, 10);
    assert_eq!(st.query(0, 3), 1 + 10 + 3 + 4);

    st.update(3, 5);
    assert_eq!(st.query(2, 3), 3 + 5);
}

#[test]
fn test_edge_cases() {
    let mut st = SegmentTree::new(&[]);
    assert_eq!(st.query(0, 0), 0);
    st.update(0, 100);
    assert_eq!(st.query(0, 0), 0);

    let mut st = SegmentTree::new(&[42]);
    assert_eq!(st.query(0, 0), 42);
    st.update(0, 100);
    assert_eq!(st.query(0, 0), 100);
}

#[test]
fn test_out_of_range() {
    let st = SegmentTree::new(&[1, 2, 3]);
    assert_eq!(st.query(0, 6), 1 + 2 + 3);
    assert_eq!(st.query(3, 5), 0);

    let mut st = SegmentTree::new(&[1, 2, 3]);
    st.update(5, 100);
    assert_eq!(st.query(0, 2), 1 + 2 + 3);
}
