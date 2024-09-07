fn main() {
    // Пример строки, в которой нужно перевернуть слова
    let input = "snow dog sun".to_string();

    let reversed_words: String = input
        .split_whitespace() // Разделяем на слова (идут через пробелы)
        .rev()
        .collect::<Vec<&str>>()
        .join(" "); // Соединяем слова пробелами

    println!("Original: {}", input);
    println!("Reversed: {}", reversed_words);
}
