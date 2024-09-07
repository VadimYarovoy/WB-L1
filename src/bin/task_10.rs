// Разработать конвейер чисел с помощью каналов.
// В первый канал с паузами пишутся числа из массива, проинициализированного 1..N.
// Первый thread или task читает из этого канала и пишет квадрат полученного числа во второй канал.
// Второй thread или task читает из второго канала и выводит в stdout.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn producer(sender: mpsc::Sender<i32>, n: i32) {
    for i in 1..=n {
        sender.send(i).expect("Failed to send number");
        println!("Produced: {}", i);
        thread::sleep(Duration::from_millis(500)); // Пауза 500 мс
    }
}

fn squarer(receiver: mpsc::Receiver<i32>, sender: mpsc::Sender<i32>) {
    while let Ok(num) = receiver.recv() {
        let squared = num * num;
        sender.send(squared).expect("Failed to send squared number");
        println!("Squared: {}", squared);
    }
}

fn consumer(receiver: mpsc::Receiver<i32>) {
    while let Ok(squared_num) = receiver.recv() {
        println!("Consumed: {}", squared_num);
    }
}

fn main() {
    let (producer_sender, producer_receiver) = mpsc::channel();
    let (squarer_sender, squarer_receiver) = mpsc::channel();

    // Запускаем задачи в отдельных потоках
    let producer_thread = thread::spawn(move || {
        producer(producer_sender, 10); // Значение N
    });

    let squarer_thread = thread::spawn(move || {
        squarer(producer_receiver, squarer_sender);
    });

    let consumer_thread = thread::spawn(move || {
        consumer(squarer_receiver);
    });

    // Ждем завершения потоков
    producer_thread.join().expect("Producer thread failed");
    squarer_thread.join().expect("Squarer thread failed");
    consumer_thread.join().expect("Consumer thread failed");
}
