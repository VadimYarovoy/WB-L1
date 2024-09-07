// Реализовать структуру-счетчик, которая будет инкрементироваться в конкурентной среде.
// По завершению программа должна выводить итоговое значение счетчика.

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicI32, Ordering};

use std::thread;

struct MutexCounter {
    value: Mutex<i32>, // Используем Mutex для защиты доступа к значению
}

impl MutexCounter {
    fn new() -> Self {
        MutexCounter {
            value: Mutex::new(0),
        }
    }

    fn increment(&self) {
        let mut num = self.value.lock().unwrap(); // Блокируем доступ к значению
        *num += 1; // Инкрементируем значение
    }

    fn get(&self) -> i32 {
        let num = self.value.lock().unwrap(); // Блокируем доступ для чтения
        *num
    }
}

struct AtomicCounter {
    value: AtomicI32, // Используем AtomicI32 для конкурентного доступа
}

impl AtomicCounter {
    fn new() -> Self {
        AtomicCounter {
            value: AtomicI32::new(0),
        }
    }

    fn increment(&self) {
        self.value.fetch_add(1, Ordering::SeqCst); // Инкрементируем значение
    }

    fn get(&self) -> i32 {
        self.value.load(Ordering::SeqCst) // Получаем текущее значение
    }
}

fn main() {
    mutex_counter();
    atomic_counter();
}

fn mutex_counter() {
    let counter = Arc::new(MutexCounter::new()); // Создаем счетчик и оборачиваем в Arc
    let mut handles = vec![];

    // Запускаем 10 потоков
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter_clone.increment(); // Инкрементируем счетчик
            }
        });
        handles.push(handle);
    }

    // Ждем завершения всех потоков
    for handle in handles {
        handle.join().unwrap();
    }

    // Выводим итоговое значение счетчика
    println!("Total value: {}", counter.get());
}

fn atomic_counter() {
    let counter = Arc::new(AtomicCounter::new()); // Создаем счетчик и оборачиваем в Arc
    let mut handles = vec![];

    // Запускаем 10 потоков
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter_clone.increment(); // Инкрементируем счетчик
            }
        });
        handles.push(handle);
    }

    // Ждем завершения всех потоков
    for handle in handles {
        handle.join().unwrap();
    }

    // Выводим итоговое значение счетчика
    println!("Total value: {}", counter.get());
}