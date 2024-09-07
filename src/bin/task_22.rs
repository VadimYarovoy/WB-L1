// Удалить i-ый элемент из вектора.

fn main() {
    aproach_one();
    aproach_two();
}

// O(n)
fn aproach_one() {
    let mut vec = vec![10, 20, 30, 40, 50];
    println!("Vec: {:?}", vec);

    let i = 2; // Индекс элемента, который нужно удалить

    if i < vec.len() {
        let removed_element = vec.remove(i);
        println!("Removed element: {}", removed_element);
    } else {
        println!("Invalid index");
    }

    println!("Vec: {:?}", vec);
}

// O(1), но без сохранения порядка
fn aproach_two() {
    let mut vec = vec![10, 20, 30, 40, 50];

    let i = 2; // Индекс элемента, который нужно удалить

    if i < vec.len() {
        let removed_element = vec.swap_remove(i);
        println!("Removed element: {}", removed_element);
    } else {
        println!("Invalid index");
    }

    println!("Vec: {:?}", vec);
}