// Дан массив чисел (инициализировать его 1..N). Используя параллельные вычисления, найти сумму квадратов этих чисел и вывести в stdout.
// Использовать только стандартную библиотеку и модули thread и mpsc.

use std::sync::mpsc;
use std::thread;

fn main() {
    // Инициализируем массив чисел от 1 до N
    let n = 100; // Размер массива
    let numbers: Vec<i32> = (1..=n).collect();

    // Количество потоков
    let num_threads = 4;

    // Определяем размер одной части
    let chunk_size = numbers.len() / num_threads;

    // Создаем канал для передачи данных из потоков
    let (tx, rx) = mpsc::channel();

    // Создаем потоки для обработки частей массива
    for i in 0..num_threads {
        let tx = tx.clone();
        let start = i * chunk_size;
        let end = if i == num_threads - 1 {
            numbers.len()
        } else {
            start + chunk_size
        };

        let chunk = numbers[start..end].to_vec();

        thread::spawn(move || {
            // Вычисляем сумму квадратов элементов в части массива
            let sum_of_squares: i32 = chunk.iter().map(|&x| x * x).sum();
            // Отправляем результат в канал
            tx.send(sum_of_squares).unwrap();
        });
    }

    // Собираем результаты из всех потоков и вычисляем общую сумму
    let total_sum: i32 = (0..num_threads).map(|_| rx.recv().unwrap()).sum();

    // Выводим результат в stdout
    println!("The sum of squares is: {}", total_sum);
}
