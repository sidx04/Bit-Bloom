use std::{collections::HashMap, hash::Hasher};
use uuid::Uuid;
use wyhash::WyHash;

struct BloomFilter {
    filter: Vec<u8>,
    size: i32,
}

impl BloomFilter {
    fn new(size: i32) -> Self {
        BloomFilter {
            filter: vec![0; size as usize],
            size,
        }
    }

    #[allow(dead_code)]
    fn display(&self) {
        println!("{:?}", self.filter)
    }

    fn add(&mut self, key: &str) {
        let index = BloomFilter::wyhash(key, self.size) % self.size;
        self.filter[(index / 8) as usize] |= 1 << (index % 8);
    }

    fn exists(&self, key: &str) -> (String, i32, bool) {
        let index = BloomFilter::wyhash(key, self.size) % self.size;
        (
            key.to_string(),
            index,
            (self.filter[(index / 8) as usize] & (1 << (index % 8))) != 0,
        )
    }

    fn wyhash(key: &str, size: i32) -> i32 {
        let mut hasher: WyHash = WyHash::with_seed(10);
        hasher.write(key.as_bytes());
        let hash_result = hasher.finish() % size as u64;
        hash_result as i32
    }
}

fn main() {
    let mut dataset: Vec<String> = Vec::with_capacity(1000);
    let mut dataset_exists: HashMap<String, bool> = HashMap::with_capacity(500);
    let mut dataset_not_exists: HashMap<String, bool> = HashMap::with_capacity(500);

    for _ in 0..500 {
        let u = Uuid::new_v4().to_string();
        dataset.push(u.clone());
        dataset_exists.insert(u.clone(), true);
    }

    for _ in 0..500 {
        let u = Uuid::new_v4().to_string();
        dataset.push(u.clone());
        dataset_not_exists.insert(u.clone(), false);
    }

    for iteration in (100..=10000).step_by(100) {
        let mut bloom_filter: BloomFilter = BloomFilter::new(iteration);

        let mut false_positive = 0;

        for k in dataset_exists.keys() {
            bloom_filter.add(k);
        }

        for key in &dataset {
            let (_, _, state) = bloom_filter.exists(key);
            if state && dataset_not_exists.contains_key(key) {
                false_positive += 1;
            }
        }

        println!(
            "False positive: (iteration: {}, value: {})",
            iteration,
            false_positive as f32 / dataset.len() as f32
        );
    }
}
