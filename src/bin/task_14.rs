// Подготовьте пример программы, которая в рантайме способна определить тип переменной, используйте std::any::*.

use std::any::Any;

// Использование downcast_ref для патерн матчинга
// Нужна ветка под каждый тип
fn print_type(value: &dyn Any) {
    if let Some(v) = value.downcast_ref::<i32>() {
        println!("Type: i32, value: {}", v);
    } else if let Some(v) = value.downcast_ref::<f64>() {
        println!("Type: f64, value: {}", v);
    } else if let Some(v) = value.downcast_ref::<String>() {
        println!("Type: String, value: {}", v);
    } else {
        println!("Unknown type");
    }
}

fn aproach_one() {
    let int_value: Box<dyn Any> = Box::new(42);
    let float_value: Box<dyn Any> = Box::new(3.14);
    let string_value: Box<dyn Any> = Box::new(String::from("Hello, Rust!"));

    print_type(&*int_value);
    print_type(&*float_value);
    print_type(&*string_value);
}

// Использование общего трейта
// Нужно имплементировать для каждого типа
pub trait Object {
    fn type_name(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}

// обзая функция для &dyn Object
pub fn type_name(x: &dyn Object) -> &str {
    x.type_name()
}

// Для проверки типов
pub fn is_of_type<T: 'static>(x: &dyn Object) -> bool {
    x.as_any().is::<T>()
}

impl Object for i32 {
    fn type_name(&self) -> &str {"i32"}
    fn as_any(&self) -> &dyn Any {self}
}

impl Object for String {
    fn type_name(&self) -> &str {"String"}
    fn as_any(&self) -> &dyn Any {self}
}

fn aproach_two() {
    let x = 21;
    println!("Type {}",type_name(&x));
    println!("Is integer {}",is_of_type::<i32>(&x));

    let x = "21".to_string();
    println!("Type {}",type_name(&x));
    println!("Is string {}",is_of_type::<String>(&x));
}

// работает для любого типа, но информация не очень понятна для человека
fn print_type_name(value: &dyn Any) {
    let type_name = value.type_id();
    println!("Type: {:?}", type_name);
}

fn aproach_three() {
    let int_value: Box<dyn Any> = Box::new(42);
    let float_value: Box<dyn Any> = Box::new(3.14);
    let string_value: Box<dyn Any> = Box::new(String::from("Hello, Rust!"));
    let vec_value: Box<dyn Any> = Box::new(vec![1, 2, 3]);

    print_type_name(&*int_value);
    print_type_name(&*float_value);
    print_type_name(&*string_value);
    print_type_name(&*vec_value);
}


// Предпочтительны 1 и 2 подходы
fn main() {
    aproach_one();
    aproach_two();
    aproach_three();
}

