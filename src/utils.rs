
use core::hash::Hash;
use std::collections::HashSet;

pub fn unique<T: Hash + std::cmp::Eq>(vec: Vec<T>) -> Vec<T> {
    vec.into_iter().collect::<HashSet<T>>().into_iter().collect()
}