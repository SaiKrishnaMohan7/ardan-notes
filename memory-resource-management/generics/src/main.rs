use std::collections::HashMap;

#[derive(Debug)]
struct HashMapBucket<K, V> {
    map: HashMap<K, Vec<V>>,
}

impl<K, V> HashMapBucket<K, V>
where
    K: Eq + std::hash::Hash,
{
    fn new() -> Self {
        HashMapBucket {
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, key: K, value: V) {
        let values = self.map.entry(key).or_insert(Vec::new());
        values.push(value);
    }
}

fn main() {
    let mut bucket = HashMapBucket::new();
    bucket.insert("hello", 1);
    bucket.insert("hello", 2);
    bucket.insert("goodbye", 3);

    println!("{bucket:?}");
}
