use std::hash::Hash;
use std::collections::HashMap;
use hashmap::HashMapU;

impl<'a, K: Eq + Hash + Clone, V: Eq + Hash + Clone> HashMapU<K, V> for HashMap<K, V> {

    fn invert(self) -> HashMap<V, K> {
        let mut invert = HashMap::new();
        for (key, value) in self.into_iter() {
            invert.insert(value, key);
        }
        return invert;
    }

    fn pick(self, keys: Vec<K>) -> HashMap<K, V> {
        let mut picked = HashMap::new();
        for element in keys.into_iter() {
            if self.contains_key(&element) {
                let v = self[&element].clone();
                picked.insert(element, v);
            }
        }
        return picked;
    }

    fn pick_by_filter<F: Fn(&K, &V) -> bool>(self, f: F) -> HashMap<K, V> {
        let mut picked = HashMap::new();
        for key in self.keys() {
            let cp_key = key.clone();
            let cp_val = self[&cp_key].clone();
            if f(&cp_key, &cp_val) { picked.insert(cp_key, cp_val); }
        }
        return picked;
    }

    fn omit(self, keys: Vec<K>) -> HashMap<K, V> {
        let mut omitted = HashMap::new();
        for (key, value) in self.into_iter() {
            if ! keys.contains(&key) { omitted.insert(key, value); }
        }

        return omitted;
    }

    fn omit_by_filter<F: Fn(&K, &V) -> bool>(self, f: F) -> HashMap<K, V> {
        let mut omitted = HashMap::new();
        for key in self.keys() {
            let cp_key = key.clone();
            let cp_val = self[&cp_key].clone();
            if ! f(&cp_key, &cp_val) { omitted.insert(cp_key, cp_val); }
        }
        return omitted;
    }

    fn defaults(self, appends: HashMap<K, V>) -> HashMap<K, V> {
        let mut origin = self.clone();
        for (key, value) in appends.into_iter() {
            if ! origin.contains_key(&key) { origin.insert(key, value); }
        }
        return origin;
    }
}
