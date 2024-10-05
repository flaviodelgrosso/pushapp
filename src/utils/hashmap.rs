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
