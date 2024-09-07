// Разработать программу нахождения расстояния между двумя точками,
// которые представлены в виде структуры Point с инкапсулированными параметрами x,y и конструктором.

fn main() {
    // Создаем точки
    let point1 = geometry::Point::new(3.0, 4.0);
    let point2 = geometry::Point::new(0.0, 0.0);

    // Вычисляем расстояние
    let distance = point1.distance(&point2);

    // Результат
    println!("Distance: {}", distance);
}

// Инкапсуляция за счет модуля
mod geometry {
    // Структура публичная
    #[derive(Debug)]
    pub struct Point {
        // Поля приватные
        x: f64,
        y: f64,
    }

    impl Point {
        // Конструктор для создания новой точки
        pub fn new(x: f64, y: f64) -> Self {
            Point { x, y }
        }

        // Метод для вычисления расстояния
        pub fn distance(&self, other: &Point) -> f64 {
            ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
        }
    }
}