
use std::collections::HashMap; 
use std::hash::Hash;

pub struct Cache<K, V> 
where 
    K: Eq+Hash+Clone,
    V: Default+Clone, 
    {
        capacity: usize,
        data: HashMap<K, V>,
    }

impl<K, V> Cache<K, V>
where 
    K: Eq+Hash+Clone,
    V: Default+Clone, 
{
    pub fn new( capacity: usize) -> Self{
        Cache{
            data: HashMap::new(),
            capacity, // rust can automatically match the name
        }
    }

    pub fn get(&mut self, key: &K) -> Option<V> {

        if let Some(value) = self.data.get(key).cloned(){
            Some(value)
        } else {
            None 
        }
    }

    pub fn insert(&mut self, key: K, value: V){
        self.data.insert(key, value); 
    }

    pub fn remove(&mut self, key: &K) -> Option<V>{

        self.data.remove(key) 

    }

    pub fn len(&self) -> usize {
        self.data.len() 
    }
}


