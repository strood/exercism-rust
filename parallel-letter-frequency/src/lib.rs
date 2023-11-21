use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut workers = Vec::new();
    let map = Arc::new(Mutex::new(HashMap::new()));

    // Join input into a single string
    let joined_input = input.join("");

    // Get our chunk size, and return an empty hashmap if it's 0
    let chunk_size = (joined_input.len() + worker_count - 1) / worker_count;
    if chunk_size == 0 {
        return HashMap::new();
    }
    // Get peekable chunks to iterate over instead of allocating
    let mut chunks = joined_input.chars().peekable();

    // Create workers, give them chunks, wait until done, and combine their hashmaps
    while chunks.peek().is_some() {
        let chunk = chunks.by_ref().take(chunk_size).collect::<String>();
        let map = Arc::clone(&map);
        workers.push(thread::spawn(move || {
            // Grab our Arc map, iterate chink chars and count them into map
            let mut map = map.lock().unwrap();
            for c in chunk.chars() {
                if c.is_alphabetic() {
                    *map.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
                }
            }
        }));
    }

    // Wait for all workers to finish
    for worker in workers {
        worker.join().unwrap();
    }

    // Return the final hashmap
    Arc::try_unwrap(map).unwrap().into_inner().unwrap()
}
