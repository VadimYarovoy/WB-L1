// Реализовать бинарный поиск встроенными методами языка.
fn main() {
    let sorted_numbers = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
    let mut target = 7;

    match sorted_numbers.binary_search(&target) {
        Ok(index) => println!("Found {} at index {}", target, index),
        Err(index) => println!("{} not found, could be inserted at index {}", target, index),
    }

    target = 6;

    match sorted_numbers.binary_search(&target) {
        Ok(index) => println!("Found {} at index {}", target, index),
        Err(index) => println!("{} not found, could be inserted at index {}", target, index),
    }
}
