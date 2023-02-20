use std::fmt::Debug;
use std::{collections::HashMap, hash::Hash};
use std::sync::{Arc, RwLock};
pub struct  InmemoryRepository<T, U> {
    store: Arc<RwLock<HashMap<T, U>>>
}

impl <T, U>InmemoryRepository<T, U> where T: Eq + Hash, U: Debug + Clone {

    pub fn new() -> Self {
        Self { store: Arc::new(RwLock::new(HashMap::new())) }
    }
    
    pub fn add_record(&self,t: T, u:U) {
        if let Ok(mut val) = self.store.write() {
            val.insert(t, u);
        } 
    }
    
    pub fn delete_record(&self,t: &T) {
        if let Ok(mut val) = self.store.write() {
            val.remove(t);
        }
    }

    pub fn get_record(&self,t: &T) -> Option<U> {
        if let Ok(val) = self.store.read() {
            if let Some(v) = val.get(t) {
                return Some(v.clone());
            }
        }
        None
    }

    pub fn get_records(&self) -> Vec<U> {
        let mut vec = vec![];
        if let Ok(val) = self.store.read() {
            for value in val.values() {
                vec.push(value.clone());
            }
        }
        
        return vec;
    }
}