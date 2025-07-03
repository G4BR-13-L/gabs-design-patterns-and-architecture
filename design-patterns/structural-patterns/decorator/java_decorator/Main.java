interface Coffee {
    double cost();
    String description();
}

class SimpleCoffee implements Coffee {

    public double cost() {
        return 2.0;
    }

    public String description() {
        return "Simple Coffee";
    }
}

abstract class CoffeeDecorator implements Coffee {

    protected Coffee decoratedCoffee;

    public CoffeeDecorator(Coffee coffee) {
        this.decoratedCoffee = coffee;
    }
}

class WithMilk extends CoffeeDecorator {

    public WithMilk(Coffee coffee) {
        super(coffee);
    }

    public double cost() {
        return decoratedCoffee.cost() + 0.5;
    }

    public String description() {
        return decoratedCoffee.description() + " + Milk";
    }
}

class WithSugar extends CoffeeDecorator {

    public WithSugar(Coffee coffee) {
        super(coffee);
    }

    public double cost() {
        return decoratedCoffee.cost() + 0.2;
    }

    public String description() {
        return decoratedCoffee.description() + " + Sugar";
    }
}

public class Main {

    public static void main(String[] args) {
        Coffee coffee = new SimpleCoffee();
        coffee = new WithMilk(coffee);
        coffee = new WithSugar(coffee);

        System.out.println(coffee.description() + ": $" + coffee.cost());
    }
}
