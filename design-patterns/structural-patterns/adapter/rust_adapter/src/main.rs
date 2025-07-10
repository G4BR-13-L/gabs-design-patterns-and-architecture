use std::f64::consts::SQRT_2;

struct RoundHole {
    radius: f64,
}

impl RoundHole {
    fn new(radius: f64) -> Self {
        RoundHole { radius }
    }

    fn fits(&self, peg: &dyn RoundPeg) -> bool {
        peg.radius() <= self.radius
    }
}

trait RoundPeg {
    fn radius(&self) -> f64;
}

struct SquarePeg {
    width: f64,
}

impl SquarePeg {
    fn new(width: f64) -> Self {
        SquarePeg { width }
    }

    fn width(&self) -> f64 {
        self.width
    }
}

struct SquarePegAdapter {
    peg: SquarePeg,
}

impl SquarePegAdapter {
    fn new(peg: SquarePeg) -> Self {
        SquarePegAdapter { peg }
    }
}

impl RoundPeg for SquarePegAdapter {
    fn radius(&self) -> f64 {
        (self.peg.width() * SQRT_2) / 2.0
    }
}

struct SimpleRoundPeg {
    radius: f64,
}

impl SimpleRoundPeg {
    fn new(radius: f64) -> Self {
        SimpleRoundPeg { radius }
    }
}

impl RoundPeg for SimpleRoundPeg {
    fn radius(&self) -> f64 {
        self.radius
    }
}

fn main() {
    let hole = RoundHole::new(5.0);
    let round_peg = SimpleRoundPeg::new(5.0);
    let small_square_peg = SquarePeg::new(5.0);
    let large_square_peg = SquarePeg::new(8.0);

    println!("Round peg fits? {}", hole.fits(&round_peg));

    let small_adapter = SquarePegAdapter::new(small_square_peg);
    println!("Small square peg fits? {}", hole.fits(&small_adapter));

    let large_adapter = SquarePegAdapter::new(large_square_peg);
    println!("Large square peg fits? {}", hole.fits(&large_adapter));
}
