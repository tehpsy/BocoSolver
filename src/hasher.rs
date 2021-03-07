use std::hash::{Hash, Hasher};

// look for hashing a hashmap in rust
pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = std::collections::hash_map::DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}