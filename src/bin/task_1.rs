// Сделать трейт Action с методом say, который должен выводить сообщение в консоль.
// Сделать структуру Person, которая содержит строковое имя.
// Сделать реализацию трейта Action для структуры Person, в котором метод say будет выводить в консоль текст “Hello, NAME” (где NAME - имя, хранимое в структуре).

fn main() {
    let person = Person::new("Vadim".to_string());
    person.say();
}

trait Action {
    fn say(&self) {
        println!("Hello, UNKNOWN");
    }
}

struct Person {
    name: String,
}

impl Person {
    fn new(name: String) -> Self {
        Person { name }
    }
}

impl Action for Person {
    fn say(&self) {
        println!("Hello, {}", self.name);
    }
}
