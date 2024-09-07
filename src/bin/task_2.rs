// Написать программу, которая параллельно рассчитает квадраты чисел, взятых из массива (массив инициализировать 1..N), и выведет их в stdout.
// Числа могут быть выведены в произвольном порядке.
// Использовать только стандартную библиотеку.

use std::sync::Arc;
use std::thread;

fn main() {
    // Инициализируем массив чисел от 1 до N
    let n = 10;
    let numbers: Vec<i32> = (1..n + 1).collect();

    // Данный подход не дает преимущества, тк вычисления не занимают много времени,
    // он проигрывает по времени схожему однопоточному коду (время на запуск и ожидание потоков).
    // Порядок завершенияпотоков не определен.
    aproach_one(numbers.clone());

    // Преимущества по прежнему нет (из-за простоты вычислений и размера массива), но тратим меньше накладных ресурсов чем в первом подходе
    // Не создаем поток на кажное значение, а делим работу между несколькими потоками.
    // А по хорошему для такого нужно использовать Rayon.
    aproach_two(numbers);

    // Также важно быть нимательным с итераторами в потоках, они в Rust ленивые,
    // поэтому из-за их неправильного применения может упасть производительность
}

fn aproach_one(nums: Vec<i32>) {
    // Возьмем Arc (как Rc, только Atomic) для безопасного разделения данных между потоками (То есть impl Sync + Send)
    let lenth = nums.len();
    let numbers = Arc::new(nums);

    let mut handles = Vec::with_capacity(lenth);

    // Для каждого значения стартуем поток
    for i in 0..numbers.len() {
        let numbers = Arc::clone(&numbers);

        // Создаем потоки и получаем handle
        let handle = thread::spawn(move || {
            let num = numbers[i];
            let square = num * num;
            println!("Square of {} is {}", num, square);
        });

        // Собираем handles
        handles.push(handle);
    }

    // Ждем завершения всех потоков через join
    for handle in handles {
        handle.join().unwrap();
    }
}

fn aproach_two(nums: Vec<i32>) {
    // Возьмем Arc (как Rc, только Atomic) для безопасного разделения данных между потоками (То есть impl Sync + Send)
    let lenth = nums.len();
    let numbers = Arc::new(nums);

    // Кол-во потоков для использования
    let num_threads = 5;

    let numbers = Arc::new(numbers);

    // Определяем размер работы для потока
    let chunk_size = numbers.len() / num_threads;

    let mut handles = Vec::with_capacity(lenth);

    for i in 0..num_threads {
        let numbers = Arc::clone(&numbers);

        let start = i * chunk_size;
        let end = if i == num_threads - 1 {
            numbers.len()
        } else {
            start + chunk_size
        };

        let handle = thread::spawn(move || {
            for num in &numbers[start..end] {
                let square = num * num;
                println!("Square of {} is {}", num, square);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
