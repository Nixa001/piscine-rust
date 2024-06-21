use std::collections::HashMap;

pub fn slices_to_map<'a, T: Eq + std::hash::Hash, U>(keys: &'a [T], values: &'a [U]) -> HashMap<&'a T, &'a U>  {
    let mut map = HashMap::new();
    let min_len = keys.len().min(values.len());

    for i in 0..min_len {
        map.insert(&keys[i], &values[i]);
    }

    map
}
