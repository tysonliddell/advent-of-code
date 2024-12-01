use std::{collections::HashMap, hash::Hash};

pub(super) fn make_counter<T: Eq + Hash>(list: Vec<T>) -> HashMap<T, usize> {
    let mut counts = HashMap::new();
    for id in list {
        *counts.entry(id).or_insert(0) += 1;
    }
    counts
}
