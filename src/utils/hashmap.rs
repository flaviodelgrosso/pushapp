use std::collections::HashMap;
use std::hash::Hash;

// A function to merge multiple optional HashMaps into one.
// It merges the additional HashMaps into the `base` HashMap.
// Each element in `others` is an `Option<&HashMap<K, V>>`.
pub fn merge<K, V>(others: &[Option<&HashMap<K, V>>]) -> HashMap<K, V>
where
  K: Eq + Hash + Clone,
  V: Clone,
{
  let mut base = HashMap::new();
  for map in others.iter().flatten() {
    for (key, value) in *map {
      base.insert(key.clone(), value.clone());
    }
  }

  base
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_merge_empty() {
    let maps: Vec<Option<&HashMap<i32, i32>>> = vec![];
    let result = merge(&maps);
    assert!(result.is_empty());
  }

  #[test]
  fn test_merge_single_empty() {
    let map: HashMap<i32, i32> = HashMap::new();
    let maps = vec![Some(&map)];
    let result = merge(&maps);
    assert!(result.is_empty());
  }

  #[test]
  fn test_merge_single_non_empty() {
    let mut map = HashMap::new();
    map.insert(1, 10);
    let maps = vec![Some(&map)];
    let result = merge(&maps);
    assert_eq!(result.len(), 1);
    assert_eq!(result[&1], 10);
  }

  #[test]
  fn test_merge_multiple_non_empty() {
    let mut first_map = HashMap::new();
    first_map.insert(1, 10);
    let mut second_map = HashMap::new();
    second_map.insert(2, 20);
    let maps = vec![Some(&first_map), Some(&second_map)];
    let result = merge(&maps);
    assert_eq!(result.len(), 2);
    assert_eq!(result[&1], 10);
    assert_eq!(result[&2], 20);
  }

  #[test]
  fn test_merge_with_none() {
    let mut first_map = HashMap::new();
    first_map.insert(1, 10);
    let maps = vec![Some(&first_map), None];
    let result = merge(&maps);
    assert_eq!(result.len(), 1);
    assert_eq!(result[&1], 10);
  }

  #[test]
  fn test_merge_overlapping_keys() {
    let mut first_map = HashMap::new();
    first_map.insert(1, 10);
    let mut second_map = HashMap::new();
    second_map.insert(1, 20);
    let maps = vec![Some(&first_map), Some(&second_map)];
    let result = merge(&maps);
    assert_eq!(result.len(), 1);
    assert_eq!(result[&1], 20);
  }
}
