// Разработать программу, которая проверяет,
// что все символы в строке уникальные (true — если уникальные, false etc).
// Функция проверки должна быть регистронезависимой.
// Например: abcd — true abCdefAaf — false aabcd — false

use std::collections::HashSet;

fn main() {
    let test1 = "abcd".to_string();
    let test2 = "abCdefAf".to_string();
    let test3 = "aabcd".to_string();

    println!("{} — {}", test1, has_unique_chars(&test1)); // true
    println!("{} — {}", test2, has_unique_chars(&test2)); // false
    println!("{} — {}", test3, has_unique_chars(&test3)); // false
}

fn has_unique_chars(input: &str) -> bool {
    // Создаем HashSet для хранения встреченных символов
    let mut seen_chars = HashSet::new();

    for c in input.chars() {
        // Приводим символ к нижнему регистру для регистронезависимости
        let lower_c = c.to_lowercase().to_string();

        // Проверяем, был ли этот символ уже встречен
        if seen_chars.contains(&lower_c) {
            return false; // Если был, возвращаем false
        }

        // Если символ новый, добавляем его в HashSet
        seen_chars.insert(lower_c);
    }

    // Если все символы уникальны, возвращаем true
    true
}
