// Дана последовательность температурных колебаний (для примера: -25.4, -27.0 13.0, 19.0, 15.5, 24.5, -21.0, 32.5).
// Объединить данные значения в интервалы с шагом в 10 градусов. Последовательность в подмножноствах не важна.
// Пример: [-30,-20):{-25.0, -27.0, -21.0}, [10, 20):{13.0, 19.0, 15.5}, [20, 30): {24.5}, etc.

use std::collections::HashMap;

fn main() {
    let temperatures = vec![-25.4, -27.0, 13.0, 19.0, 15.5, 24.5, -21.0, 32.5];
    let mut intervals: HashMap<(i32, i32), Vec<f64>> = HashMap::new();

    for &temp in &temperatures {
        // Определяем границы интервала
        let lower_bound = ((temp / 10.0) as f64 ).floor() as i32 * 10;
        let upper_bound = lower_bound + 10;

        // Добавляем температуру в соответствующий интервал
        intervals.entry((lower_bound, upper_bound))
            .or_insert_with(Vec::new)
            .push(temp);
    }

    // HashMap -> Vec
    let mut intervals = intervals.into_iter().collect::<Vec<_>>();

    // сортируем интервалы быстрой сортировкой
    intervals.sort_unstable_by_key(|x| x.0);

    // Выводим результаты
    for ((lower, upper), temps) in &intervals {
        println!("[{}, {}): {:?}", lower, upper, temps);
    }
}
