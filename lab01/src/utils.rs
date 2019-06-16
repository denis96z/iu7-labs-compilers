use std::hash::Hash;

use crate::types;

pub fn make_empty_set<T>() -> types::Set<T>
where
    T: Eq + Hash + Ord,
{
    vec![].into_iter().collect::<types::Set<T>>()
}

pub fn make_set_from_vec<T>(v: Vec<T>) -> types::Set<T>
where
    T: Eq + Hash + Ord,
{
    v.into_iter().collect::<types::Set<T>>()
}

pub fn make_sets_union<T>(s1: &types::Set<T>, s2: &types::Set<T>) -> types::Set<T>
where
    T: Eq + Hash + Ord + Clone,
{
    s1.union(&s2)
        .into_iter()
        .cloned()
        .collect::<types::Set<T>>()
}

pub fn merge_vectors<T>(mut v1: Vec<T>, mut v2: Vec<T>) -> Vec<T> {
    v1.append(&mut v2);
    return v1;
}

pub fn find_diff<T>(v1: &[T], v2: &[T]) -> Vec<T>
where
    T: Eq + Copy,
{
    let mut result = Vec::new();
    for item in v1 {
        if !v2.contains(item) {
            result.push(*item);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_diff() {
        let v1 = vec![1, 2, 3];
        let v2 = vec![1, 2];

        assert_eq!(super::find_diff(&v1, &v2), vec![3]);
    }
}
