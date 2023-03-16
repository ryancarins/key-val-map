#![feature(map_try_insert)]

use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

pub struct KeyValMap<K: Eq + Hash + Clone, V: Eq + Hash + Clone> {
    keys: HashMap<Rc<K>, Rc<V>>,
    vals: HashMap<Rc<V>, Rc<K>>,
}

#[derive(Debug)]
pub enum KeyValMapError {
    InsertError,
}

impl<K: Eq + Hash + Clone, V: Eq + Hash + Clone> KeyValMap<K, V> {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            vals: HashMap::new(),
        }
    }
    pub fn insert(&mut self, key: K, val: V) -> Result<(), KeyValMapError> {
        if self.keys.contains_key(&key) || self.vals.contains_key(&val) {
            return Err(KeyValMapError::InsertError);
        }
        let keyrc = Rc::new(key);
        let valrc = Rc::new(val);
        self.keys.insert(keyrc.clone(), valrc.clone());
        self.vals.insert(valrc, keyrc);
        Ok(())
    }

    pub fn get_by_key(&self, key: &K) -> Option<V> {
        let inner_val = self.keys.get(key);
        if inner_val.is_none() {
            return None;
        }

        Some((**inner_val.unwrap()).clone())
    }

    pub fn get_by_val(&self, val: &V) -> Option<K> {
        let inner_key = self.vals.get(val);
        if inner_key.is_none() {
            return None;
        }

        Some((**inner_key.unwrap()).clone())
    }

    pub fn remove_by_key(&mut self, key: &K) -> Option<V> {
        let val = self.keys.remove(key);
        if val.is_none() {
            return None;
        }
        self.vals.remove(&val.clone().unwrap());
        Some((*val.unwrap()).clone())
    }

    pub fn remove_by_val(&mut self, val: &V) -> Option<K> {
        let key = self.vals.remove(val);
        if key.is_none() {
            return None;
        }
        self.keys.remove(&key.clone().unwrap());
        Some((*key.unwrap()).clone())
    }

    pub fn to_key_vec(&self) -> Vec<K> {
        let mut key_vec: Vec<K> = Vec::new();
        for key in self.keys.keys() {
            key_vec.push((*key.clone()).clone());
        }
        return key_vec;
    }

    pub fn to_val_vec(&self) -> Vec<V> {
        let mut val_vec: Vec<V> = Vec::new();
        for key in self.vals.keys() {
            val_vec.push((*key.clone()).clone());
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
        return self.contains_val(val) && self.contains_key(key);
    }
}
