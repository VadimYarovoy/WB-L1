// Разработать программу, которая переворачивает подаваемую на ход строку (например: «главрыба — абырвалг»).
// Символы могут быть unicode.

fn main() {
    let input = "🤖главрыба🌝";

    // Разбиваем на отдельные UTF8, переворачивает, собираем
    let reversed: String = input.chars().rev().collect();

    println!("Original: {}", input);
    println!("Reversed: {}", reversed);
}
