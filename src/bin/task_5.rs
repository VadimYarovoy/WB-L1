// Программа должна аккуратно завершаться по нажатию Ctrl+C. Выбрать и обосновать способ завершения работы всех воркеров.
// Использовать tokio и flume (или другую аналогичную библиотеку для spmc/mpmc-каналов).

use flume::{unbounded, Receiver};
use std::sync::Arc;
use tokio::signal;
use tokio::task;

#[derive(Debug)]
enum Message {
    Work(String),
    Shutdown,
}

async fn worker(id: usize, receiver: Arc<Receiver<Message>>) {
    while let Ok(message) = receiver.recv_async().await {
        match message {
            Message::Work(data) => {
                println!("Worker {} processing: {}", id, data);
                // Добавим задержку для имитации работы
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
            Message::Shutdown => {
                println!("Worker {} shutting down.", id);
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let (sender, receiver) = unbounded();
    let receiver = Arc::new(receiver);
    let sender = Arc::new(sender);

    // Запускаем несколько воркеров
    let num_workers = 4;
    let mut handles = vec![];

    for id in 0..num_workers {
        let receiver_clone = Arc::clone(&receiver);
        let handle = task::spawn(async move {
            worker(id, receiver_clone).await;
        });
        handles.push(handle);
    }

    // Запускаем задачу для обработки сигналов

    // Выбрано использование tokio::signal::ctrl_c.
    // Это позволяет программе реагировать на системные сигналы,
    // что является стандартным способом обработки завершения работы в асинхронных приложениях
    let sender_clone = Arc::clone(&sender); // Клонируем sender для использования в обработчике сигналов
    let signal_handle = task::spawn(async move {
        signal::ctrl_c().await.expect("Failed to install Ctrl+C handler");
        println!("Received Ctrl+C, shutting down...");
        // Отправляем сигнал завершения всем воркерам
        for _ in 0..num_workers {
            sender_clone.send(Message::Shutdown).expect("Failed to send shutdown message");
        }
    });

    // Отправляем рабочие сообщения
    for i in 0..10 {
        sender.send(Message::Work(format!("Task {}", i))).expect("Failed to send work message");
    }

    // Ждем завершения всех воркеров
    for handle in handles {
        handle.await.expect("Worker task failed");
    }

    // Ждем завершения обработки сигнала
    signal_handle.await.expect("Signal handler task failed");

    println!("All workers have been shut down.");
}
