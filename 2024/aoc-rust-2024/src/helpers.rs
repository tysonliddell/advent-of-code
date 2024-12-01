use std::{collections::HashMap, hash::Hash};

pub fn make_counter<T: Eq + Hash>(data: Vec<T>) -> HashMap<T, usize> {
    let mut counts = HashMap::new();
    for val in data {
        *counts.entry(val).or_insert(0) += 1;
    }
    counts
}
