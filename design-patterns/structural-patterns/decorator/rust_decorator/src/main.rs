trait Coffee {
    fn cost(&self) -> f64;
    fn description(&self) -> String;
}

struct SimpleCoffee;

impl Coffee for SimpleCoffee {
    fn cost(&self) -> f64 {
        2.0
    }

    fn description(&self) -> String {
        "Simple Coffee".to_string()
    }
}

struct WithMilk<T: Coffee> {
    coffee: T,
}

impl<T: Coffee> Coffee for WithMilk<T> {
    fn cost(&self) -> f64 {
        self.coffee.cost() + 0.5
    }

    fn description(&self) -> String {
        format!("{} + Milk", self.coffee.description())
    }
}

struct WithSugar<T: Coffee> {
    coffee: T,
}

impl<T: Coffee> Coffee for WithSugar<T> {
    fn cost(&self) -> f64 {
        self.coffee.cost() + 0.2
    }

    fn description(&self) -> String {
        format!("{} + Sugar", self.coffee.description())
    }
}

fn main() {
    let coffee = SimpleCoffee;

    let coffee_with_milk = WithMilk { coffee };
    let coffe_with_milk_and_sugar = WithSugar {
        coffee: coffee_with_milk,
    };

    println!(
        "{}: R${}",
        coffe_with_milk_and_sugar.description(),
        coffe_with_milk_and_sugar.cost()
    );
}
