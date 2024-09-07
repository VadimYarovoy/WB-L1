// Разработать программу, которая перемножает, делит, складывает,
// вычитает две числовых переменных a,b, значение которых > 2^20.

use num_bigint::{BigInt, Sign};

fn main() {
    println!("===aproach_one===");
    aproach_one();
    println!("\n\n===aproach_two===");
    aproach_two();
    println!("\n\n===aproach_three===");
    aproach_three();
}

// Простой способ
// используем типы языка, опишем подходящие нам
// i32 - диапазон значений от −2^31 до 2^31 − 1 (Для умножения не подойдет)
// u32 - может хранить только неотрицательные значения, то есть от 0 до 2^32 − 1 (Для умножения не подойдет)
// u64 - может хранить только неотрицательные значения, то есть от 0 до 2^64 − 1 (Очень огрничен для умножения больших чисел друг на друга)
// u128 (Не апаратный тип, работает медленнее) - может хранить только неотрицательные значения, то есть от 0 до 2^128 − 1 (Лучший из всех вариантов, представленных в стандартых типах)
fn aproach_one() {
    let a: u128 = (2u64.pow(20) * 200) as u128;
    let b: u128 = (2u64.pow(20) * 2) as u128;

    println!("Sum {}", a + b);
    // При вычитании нужно проверя какое значение больше, чтобы не получить отрицательный ответ
    if a > b {
        println!("Sub {}", a - b);
    } else {
        println!("Sub -{}", b - a);
    }
    println!("Div {}", a / b);
    println!("Mul {}", a * b);

    // Можно даже больше, но есть предел и он давольно не велик
    println!("Mul {}", a.pow(4));
}

// Сложный и более медленный способ, но с поддержкой больших чисел
// Мы будем оганичены только тем что можем иметь максимум usize цифр в числе (String это Vec<u8>)
// Реализуем только один операнд, потому что способ 3 будет делать это за нас
fn aproach_two() {
    let str1 = String::from("1235421415454545454545454544");
    let str2 = String::from("1714546546546545454544548544544545");

    let num1 = str1.trim_start_matches('-');
    let num2 = str2.trim_start_matches('-');

    println!("Mul {}", multiply(num1, num2));
}

fn multiply(num1: &str, num2: &str) -> String {
    let len1 = num1.len();
    let len2 = num2.len();
    if len1 == 0 || len2 == 0 {
        return "0".to_string();
    }

    // Вектор для хранения результата в обратном порядке
    let mut result = vec![0; len1 + len2];

    // Индексы для позиционирования в результате
    let mut i_n1 = 0;
    let mut i_n2;

    // Проходим по num1 справа налево
    for i in (0..len1).rev() {
        let n1 = num1.chars().nth(i).unwrap().to_digit(10).unwrap() as usize;
        let mut carry = 0;
        i_n2 = 0;

        // Проходим по num2 справа налево
        for j in (0..len2).rev() {
            let n2 = num2.chars().nth(j).unwrap().to_digit(10).unwrap() as usize;

            // Умножаем и добавляем к текущей позиции
            let sum = n1 * n2 + result[i_n1 + i_n2] + carry;
            carry = sum / 10;
            result[i_n1 + i_n2] = sum % 10;

            i_n2 += 1;
        }

        // Сохраняем остаток в следующей ячейке
        if carry > 0 {
            result[i_n1 + i_n2] += carry;
        }

        i_n1 += 1;
    }

    // Игнорируем нули справа
    let mut i = result.len() - 1;
    while result[i] == 0 {
        if i == 0 { break; } // Чтобы избежать переполнения
        i -= 1;
    }

    // Если все нули - значит одно из чисел было 0
    if i == 0 && result[0] == 0 {
        return "0".to_string();
    }

    // Генерируем строку результата
    let mut s = String::new();
    while i != usize::MAX {
        s.push_str(&result[i].to_string());
        if i == 0 { break; } // Чтобы избежать переполнения
        i -= 1;
    }

    s
}

// используем сторонний крейт, ограничение usize цифр в числе, логика в нутри как и у второго способа
fn aproach_three() {
    let x = BigInt::new(Sign::Plus, vec![1; 100]); // для u128 максимально 39 цифр, а тут 100
    let y = BigInt::new(Sign::Plus, vec![1; 100]);
    println!("Sum: {}", x.clone() + y.clone());
    println!("Sub: {}", x.clone() - y.clone());
    println!("Mul: {}", x.clone() * y.clone());
    println!("Div: {}", x.clone() / y.clone());
}