#![allow(dead_code)]
#![allow(unused_variables)]

use std::time::Instant;
use std::collections::HashMap;
use std::num::Wrapping;

// Initial capacity for our hashmap when we create an instance with new constructor
const INITIAL_CAPACITY: usize = 15;  
 
#[derive(Default, Clone, Debug, PartialEq, Copy)]
struct Pair<K, V> {
    key: K,
    value: V,
    // Will be very helpful because we want to track the pair to know
    // whether the key-value pair occupies the index or not. 
    is_occupied: bool 
}

// Structure of our Hashmap. 
// Storing all Key-Value associatives in Vec so that we can do any operation with the data.
#[derive(Debug, Default)]
struct MyHashMap<K, V> {
    bucket: Vec<Pair<K, V>>
}

impl<K, V> MyHashMap<K, V> 
where 
    K: Default + Clone + HashIt + std::fmt::Debug + PartialEq,
    V: Default + Clone + std::fmt::Debug
{
    fn new() -> Self {
        Self {
            bucket: vec![Pair::default(); INITIAL_CAPACITY] // Container of pairs 
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            bucket: vec![Pair::default(); capacity]
        }
    }

    fn increase_capacity(&mut self) {
        let mut new_map = Self::with_capacity(self.bucket.len() * 2);

        for pair in self.bucket.iter() {
            new_map.insert(pair.key.clone(), pair.value.clone());
            }

        *self = new_map;
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {

        let mut index = key.hash() % self.bucket.len();
        
        if !self.bucket[index].is_occupied {
            self.bucket[index].key = key;
            self.bucket[index].value = value;
            self.bucket[index].is_occupied = true;
            return None;

        } 

        // Using open addressing (linear probing) to make insertions and give a pair an index. 
        // Linear probing is quite simple and effective 
        // as it can handle collisions well by just shifting the pair to unoccupied index.
        // Chain probing can be used to link keys under the same hash. Can be accessed through key as same as linear probing

        //   H(k)  -> Index
        // |--------------------------------|
        // | (k,v) | (k,v) | (k,v) | (k,v)  | ------> Linear probing 
        // |--------------------------------|


        //   H(k)  -> Index
        // |--------------------------------|
        // | (k,v) | (k,v) | (k,v) | (k,v)  | 
        // |--------------------------------|
        //    /\
        //    ||
        //    ||
        // |-------|
        // | (k,v) |        ------> Chain probing 
        // |-------|

        let start = index;
        loop {
            index = (index + 1) % self.bucket.len();

            if self.bucket[index].key == key {
                let old_value = self.bucket[index].value.clone();
                self.bucket[index].value = value;
                return Some(old_value);
            }

            if !self.bucket[index].is_occupied {
                self.bucket[index].key = key;
                self.bucket[index].value = value;
                self.bucket[index].is_occupied = true;
                return None;
            }

            if start == index {
                self.increase_capacity();
            }
        }      
    } 

    fn extend(&mut self, how_much: usize) {
        self.bucket.reserve_exact(how_much);
    }

    fn remove(&mut self, key: K) -> Option<Pair<K, V>>{

        let index = key.hash() % self.bucket.len();

        if self.bucket[index].is_occupied {
            let removed = &self.bucket[index].clone();
            self.bucket[index].key = K::default();
            self.bucket[index].value = V::default();
            self.bucket[index].is_occupied = false;
            return Some(removed.clone());
        } else {
            panic!("Given Key does not exist.")
        }

    }

    fn get(&self, key: &K) -> Option<&V> {

        let index = key.hash() % self.bucket.len();

        if self.bucket[index].is_occupied {
            Some(&self.bucket[index].value)
        } else {
            None
        }
    }

    fn print_it(&self) {
        for (index, pair) in self.bucket.iter().enumerate() {   
            if pair.is_occupied {      
            println!("{:?}: {:?}   index: {index}", pair.key, pair.value);
            } else {
                println!("---"); // Unoccupied
            }
        }
    }
}

trait HashIt {

    // The key must not be moved while hashing it. So reference to key is passed.
    fn hash(&self) -> usize; 
}

impl HashIt for usize {
    fn hash(&self) -> usize {
        let a = 2654435769;
        let product = (self.wrapping_mul(a)) >> (64 - 32);

        product as usize        
    }
}

impl HashIt for String {
    fn hash(&self) -> usize {
        // Daniel J. Bernstein's djb2 algorithm
        // Better suited especially for strings
        // Reason to use 5381 is that it is seemed to have fewer collisions and significant change in the hash if a bit flips 
        // which are essential to be considered a good hash function.
        let mut hash: usize = 5381;
 
        for c in self.bytes() {
            hash = (hash << 5).wrapping_add(hash).wrapping_add(c as usize);
        }
        hash
    }
}

fn main() {

    let mut my_hashmap = MyHashMap::<String, u64>::new();
    let start = Instant::now();
    for i in 0 .. 10000 {
        my_hashmap.insert(format!("{}",i), i*i);
    }
    let end = start.elapsed().as_secs_f32();
    println!("MY HASHMAP: {}secs", end);

    println!("__________");

    let start = Instant::now();
    let mut std_hashmap = HashMap::new();
    
    for i in 0 .. 10000 {
        std_hashmap.insert(format!("{}",i), i*i);
    }
    let end = start.elapsed().as_secs_f32();
    println!("STD HASHMAP: {}secs", end);


}