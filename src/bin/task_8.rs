// Реализовать конкурентную запись данных в map несколькими способами: Mutex с HashMap, DashMap

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

use dashmap::DashMap;

fn main() {
    hash_map_with_arc();
    println!("----");
    dashmap();
}

fn hash_map_with_arc() {
    let map: Arc<Mutex<HashMap<i32, i32>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];

    for i in 0..10 {
        let map_clone = Arc::clone(&map);
        let handle = thread::spawn(move || {
            let mut map = map_clone.lock().unwrap();
            map.insert(i, i * 2);
            println!("Inserted: {} -> {}", i, i * 2);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Выводим содержимое карты
    let map = map.lock().unwrap();
    for (key, value) in map.iter() {
        println!("{}: {}", key, value);
    }
}

fn dashmap() {
    let map: Arc<DashMap<i32, i32>> = Arc::new(DashMap::new());
    let mut handles = vec![];

    for i in 0..10 {
        // Используем ссылку на оригинальный DashMap
        let map_ref = Arc::clone(&map);
        let handle = thread::spawn(move || {
            map_ref.insert(i, i * 2);
            println!("Inserted: {} -> {}", i, i * 2);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Выводим содержимое карты
    for r in map.iter() {
        println!("{}: {}", r.key(), r.value());
    }
}
