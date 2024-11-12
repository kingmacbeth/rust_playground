trait PaymentStrategy {
    fn pay(&self, from: String, to: String);
}

struct CashStrategy;

impl PaymentStrategy for CashStrategy {
    fn pay(&self, from: String, to: String) {
        println!("{} paid {} using cash.", from, to);
    }
}

struct CreditCardStrategy;

impl PaymentStrategy for CreditCardStrategy {
    fn pay(&self, from: String, to: String) {
        println!("{} paid {} using credit card.", from, to);
    }
}

struct VendingMachine<T: PaymentStrategy> {
    payment_strategy: T,
}

impl<T: PaymentStrategy> VendingMachine<T> {
    pub fn new(payment_strategy: T) -> Self {
        Self { payment_strategy }
    }

    pub fn payment_strategy(&self, from: String, to: String) {
        self.payment_strategy.pay(from, to);
    }
}

fn main() {
    {
        let vending_machine = VendingMachine::new(CashStrategy);
        vending_machine.payment_strategy(String::from("Renato"), String::from("Marcilio"));
    }

    {
        let vending_machine = VendingMachine::new(CreditCardStrategy);
        vending_machine.payment_strategy(String::from("Renato"), String::from("Marcilio"));
    }
}
