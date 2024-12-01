use std::{collections::HashMap, hash::Hash};

pub fn make_counter<I, T>(data: I) -> HashMap<T, usize>
where
    I: Iterator<Item = T>,
    T: Eq + Hash,
{
    let mut counts = HashMap::new();
    for val in data {
        *counts.entry(val).or_insert(0) += 1;
    }
    counts
}
