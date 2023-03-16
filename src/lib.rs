#![feature(map_try_insert)]

use std::collections::HashMap;
use std::hash::Hash;

pub struct KeyValMap<K: Eq + Hash + Clone, V: Eq + Hash + Clone> {
    keys: HashMap<K, V>,
    vals: HashMap<V, K>,
}

#[derive(Debug)]
pub enum KeyValMapError {
    InsertError
}

impl<K: Eq + Hash + Clone, V: Eq + Hash + Clone> KeyValMap<K, V> {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            vals: HashMap::new(),
        }
    }
    pub fn insert(&mut self, key: K, val: V) -> Result<(),KeyValMapError> {
        if self.keys.contains_key(&key) || self.vals.contains_key(&val) {
            return Err(KeyValMapError::InsertError);
        }
        self.keys.insert(key.clone(), val.clone());
        self.vals.insert(val, key);
        Ok(())
    }

    pub fn get_by_key(&self, key: &K) -> Option<V> {
        self.keys.get(key).cloned()
    }

    pub fn get_by_val(&self, val: &V) -> Option<K> {
        self.vals.get(val).cloned()
    }

    pub fn remove_by_key(&mut self, key: &K) -> Option<V> {
        let val = self.keys.remove(key);
        if val.is_none() {
            return None;
        }
        self.vals.remove(&val.clone().unwrap());
        return val;
    }

    pub fn remove_by_val(&mut self, val: &V) -> Option<K> {
        let key = self.vals.remove(val);
        if key.is_none() {
            return None;
        }
        self.keys.remove(&key.clone().unwrap());
        return key;
    }

    pub fn to_key_vec(&self) -> Vec<K> {
        let mut key_vec: Vec<K> = Vec::new();
        for key in self.keys.keys() {
            key_vec.push(key.clone());
        }
        return key_vec;
    }

    pub fn to_val_vec(&self) -> Vec<V> {
        let mut val_vec: Vec<V> = Vec::new();
        for key in self.vals.keys() {
            val_vec.push(key.clone());
        }
        return val_vec;
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.keys.contains_key(key)
    }

    pub fn contains_val(&self, val: &V) -> bool {
        self.vals.contains_key(val)
    }

    pub fn contains(&self, key: &K, val: &V) -> bool {
        return self.contains_val(val) && self.contains_key(key)
    }
}