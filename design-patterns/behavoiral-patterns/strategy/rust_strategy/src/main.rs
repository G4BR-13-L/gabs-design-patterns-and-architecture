trait PaymentStrategy {
    fn pay(&self, amount: f64);
}

struct CreditCardPayment {
    card_number: String,
}

impl PaymentStrategy for CreditCardPayment {
    fn pay(&self, amount: f64) {
        println!("Paid ${} using Credit Card: {}", amount, self.card_number);
    }
}

#[allow(dead_code)]
enum PixKeyType {
    EMAIL,
    CPF,
    CNPJ,
    RANDOM,
    PHONE,
}

impl ToString for PixKeyType {
    fn to_string(&self) -> String {
        match self {
            PixKeyType::CPF => String::from("CPF"),
            PixKeyType::EMAIL => String::from("EMAIL"),
            PixKeyType::CNPJ => String::from("CNPJ"),
            PixKeyType::PHONE => String::from("PHONE"),
            PixKeyType::RANDOM => String::from("RANDOM"),
        }
    }
}

struct PixPayment {
    key: String,
    key_type: PixKeyType,
}

impl PaymentStrategy for PixPayment {
    fn pay(&self, amount: f64) {
        println!(
            "Paid ${} using Pix key: {}, of type: {}",
            amount,
            self.key,
            self.key_type.to_string()
        );
    }
}

struct ShoppingCart<'a> {
    strategy: Box<dyn PaymentStrategy + 'a>,
}

impl<'a> ShoppingCart<'a> {
    fn new(strategy: Box<dyn PaymentStrategy + 'a>) -> Self {
        ShoppingCart { strategy }
    }

    fn checkout(&self, amount: f64) {
        self.strategy.pay(amount);
    }
}

fn main() {
    let pix = Box::new(PixPayment {
        key: "user@email.com".to_string(),
        key_type: PixKeyType::EMAIL,
    });

    let credit_card = Box::new(CreditCardPayment {
        card_number: "1234-5678-9101-1121-3141".to_string(),
    });

    let cart1 = ShoppingCart::new(pix);

    let cart2 = ShoppingCart::new(credit_card);

    cart1.checkout(100.00);
    cart2.checkout(213.00);
}
