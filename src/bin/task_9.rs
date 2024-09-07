// Дана переменная int64. Разработать программу которая устанавливает i-й бит в 1 или 0.

fn main() {
    let mut number: i64 = 0; // Начальное значение
    let bit_position: u32 = 4; // Позиция бита, который нужно установить
    let set_to_one: bool = true; // Установить бит в 1 или 0

    // Устанавливаем i-й бит
    number = set_bit(number, bit_position, set_to_one);
    println!(
        "After setting bit {} in {} pos: {}",
        bit_position, set_to_one as i64, number
    );

    // Установим i-й бит в 0
    let set_to_zero: bool = false;
    number = set_bit(number, bit_position, set_to_zero);
    println!(
        "After setting bit {} in {} pos: {}",
        bit_position, set_to_zero as i64, number
    );
}

fn set_bit(num: i64, i: u32, value: bool) -> i64 {
    if value {
        // Устанавливаем i-й бит в 1
        num | (1 << i)
    } else {
        // Устанавливаем i-й бит в 0
        num & !(1 << i)
    }
}
