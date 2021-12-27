use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

pub fn assert<T: Eq + Hash + Debug, const N: usize>(left: HashSet<T>, right: [T; N]) {
    assert_eq!(left, right.into());
}
