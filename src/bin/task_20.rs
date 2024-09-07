// Реализовать паттерн «адаптер» на любом примере.

// Интерфейс для обработки платежей
trait PaymentProcessor {
    fn process_payment(&self, amount: f64);
}

// Класс, который мы хотим адаптировать
struct OldPaymentSystem;

impl OldPaymentSystem {
    fn make_payment(&self, amount: f64) {
        println!("Payment proccesing in old system: ${:.2}", amount);
    }
}

// Адаптер для старой платежной системы
struct OldPaymentAdapter {
    old_system: OldPaymentSystem,
}

impl PaymentProcessor for OldPaymentAdapter {
    fn process_payment(&self, amount: f64) {
        self.old_system.make_payment(amount);
    }
}

// Новый класс платежной системы
struct NewPaymentSystem;

impl PaymentProcessor for NewPaymentSystem {
    fn process_payment(&self, amount: f64) {
        println!("Payment proccesing in new system: ${:.2}", amount);
    }
}

fn main() {
    let new_payment = NewPaymentSystem;
    new_payment.process_payment(100.0);

    let old_payment_adapter = OldPaymentAdapter {
        old_system: OldPaymentSystem,
    };
    old_payment_adapter.process_payment(50.0);
}
