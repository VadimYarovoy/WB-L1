// Разработать программу, которая будет последовательно отправлять значения в канал, а с другой стороны канала — читать.
// По истечению N секунд программа должна завершаться.

use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    // Указываем количество секунд, после которых программа завершится
    let duration_in_seconds: u64 = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "10".to_string()) // По умолчанию 10 секунд
        .parse()
        .expect("Incorrect time value");

    // Создаем канал
    let (tx, rx) = mpsc::channel();

    // Запускаем поток для отправки значений в канал
    thread::spawn(move || {
        for count in 0.. {
            let message = format!("Message: {}", count);
            if tx.send(message).is_err() {
                // Если канал закрыт, выходим из цикла
                break;
            }
            thread::sleep(Duration::from_secs(1)); // Задержка в 1 секунду
        }
    });

    // Запускаем поток для чтения значений из канала
    thread::spawn(move || loop {
        match rx.recv() {
            Ok(message) => {
                println!("Received: {}", message);
            }
            Err(_) => {
                println!("Chanel slosed, stop working.");
                break;
            }
        }
    });

    // Ждем указанное количество секунд
    let start_time = Instant::now();
    while start_time.elapsed() < Duration::new(duration_in_seconds, 0) {
        thread::sleep(Duration::from_millis(100)); // Проверяем каждые 100 мс
    }
}
