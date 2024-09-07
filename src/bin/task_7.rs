use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use tokio_util::sync::CancellationToken;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    // Запуск задач с использованием разных методов остановки
    stop_with_channel().await;
    stop_with_cancellation_token().await;
    stop_with_flag().await;
}

async fn stop_with_channel() {
    let (tx, mut rx) = mpsc::channel(32);

    // Запускаем задачу
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            println!("Channel Task Received: {}", msg);
        }
        println!("Channel Task: Channel closed, task exiting.");
    });

    // Отправляем сообщения
    tx.send("Hello from Channel").await.unwrap();
    tx.send("World from Channel").await.unwrap();

    // Закрываем канал
    drop(tx);

    // Ждем немного, чтобы увидеть вывод
    sleep(Duration::from_secs(1)).await;
}

async fn stop_with_cancellation_token() {
    let token = CancellationToken::new();
    let token_clone = token.clone();

    // Запускаем задачу
    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = token_clone.cancelled() => {
                    println!("Cancellation Token Task: Task cancelled.");
                    break;
                }
                _ = sleep(Duration::from_secs(1)) => {
                    println!("Cancellation Token Task: Working...");
                }
            }
        }
    });

    // Ждем немного, чтобы увидеть вывод
    sleep(Duration::from_secs(3)).await;

    // Отменяем задачу
    token.cancel();

    // Ждем завершения задачи
    sleep(Duration::from_secs(1)).await;
}

async fn stop_with_flag() {
    let running = Arc::new(Mutex::new(true));
    let running_clone = running.clone();

    // Запускаем задачу
    tokio::spawn(async move {
        while *running_clone.lock().unwrap() {
            println!("Flag Task: Working...");
            sleep(Duration::from_secs(1)).await;
        }
        println!("Flag Task: Task exiting.");
    });

    // Ждем немного, чтобы увидеть вывод
    sleep(Duration::from_secs(3)).await;

    // Устанавливаем флаг завершения
    *running.lock().unwrap() = false;

    // Ждем завершения задачи
    sleep(Duration::from_secs(1)).await;
}
