// Strategy interface
interface PaymentStrategy {
    void pay(double amount);
}

// Concrete strategy: Credit Card
class CreditCardPayment implements PaymentStrategy {

    private String cardNumber;

    public CreditCardPayment(String cardNumber) {
        this.cardNumber = cardNumber;
    }

    public void pay(double amount) {
        System.out.println(
            "Paid $" + amount + " using Credit Card: " + cardNumber
        );
    }
}

enum PixKeyType {
    EMAIL,
    CPF,
    CNPJ,
    PHONE,
    RANDOM,
}

// Concrete strategy: PayPal
class PixPayment implements PaymentStrategy {

    private String key;
    private PixKeyType keyType;

    public PixPayment(String key, PixKeyType keyType) {
        this.key = key;
        this.keyType = keyType;
    }

    public void pay(double amount) {
        System.out.println(
            "Paid " +
            amount +
            " using Pix key: " +
            this.key +
            ", of type: " +
            this.keyType.name()
        );
    }
}

// Context
class ShoppingCart {

    private PaymentStrategy strategy;

    public ShoppingCart(PaymentStrategy strategy) {
        this.strategy = strategy;
    }

    public void checkout(double amount) {
        strategy.pay(amount);
    }
}

public class Main {

    public static void main(String[] args) {
        ShoppingCart cart1 = new ShoppingCart(
            new CreditCardPayment("1234-5678-9101-1121-3141")
        );
        cart1.checkout(99.99);

        ShoppingCart cart2 = new ShoppingCart(
            new PixPayment("123.456.789.10", PixKeyType.CPF)
        );
        cart2.checkout(49.50);
    }
}
