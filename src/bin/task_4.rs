// Реализовать постоянную запись данных в канал (главный поток).
// Реализовать набор из N воркеров, которые читают произвольные данные из канала и выводят в stdout.
// Необходима возможность выбора количества воркеров при старте.

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Читаем количество воркеров из аргументов командной строки
    let num_workers: usize = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "2".to_string()) // По умолчанию 2 воркера
        .parse()
        .expect("Invalid worcker number");

    // Создаем канал
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    let mut handles = vec![];
    for i in 0..num_workers {
        let rx = Arc::clone(&rx);
        let handle = thread::Builder::new()
            .name(format!("worker-{}", i)) // Задаем имя потока
            .spawn(move || {
                let thread = thread::current();
                let thread_name = thread.name().unwrap_or("unknown");
                loop {
                    let data = rx.lock().unwrap().recv();
                    match data {
                        Ok(message) => {
                            println!("{}: Recived msg: {}", thread_name, message);
                        }
                        Err(_) => {
                            println!("{}: Chenel closed, worker stopped.", thread_name);
                            break;
                        }
                    }
                }
            })
            .expect("Failed to start worker");
        handles.push(handle);
    }

    let mut count = 0;
    loop {
        let message = format!("Message {}", count);
        tx.send(message).expect("Failed to send message");
        count += 1;
        thread::sleep(Duration::from_secs(1));
    }

    // Ждем завершения воркеров (никогда не произойдет, тк выше loop без условия выхода)
    // for handle in handles {
    //     handle.join().unwrap();
    // }
}
