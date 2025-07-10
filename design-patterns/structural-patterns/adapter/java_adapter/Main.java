interface RoundPeg {
    double getRadius();
}

class SimpleRoundPeg implements RoundPeg {

    private final double radius;

    public SimpleRoundPeg(double radius) {
        this.radius = radius;
    }

    public double getRadius() {
        return radius;
    }
}

class SquarePeg {

    private final double width;

    public SquarePeg(double width) {
        this.width = width;
    }

    public double getWidth() {
        return width;
    }
}

class SquarePegAdapter implements RoundPeg {

    private final SquarePeg peg;

    public SquarePegAdapter(SquarePeg peg) {
        this.peg = peg;
    }

    public double getRadius() {
        return (peg.getWidth() * Math.sqrt(2)) / 2;
    }
}

class RoundHole {

    private final double radius;

    public RoundHole(double radius) {
        this.radius = radius;
    }

    public boolean fits(RoundPeg peg) {
        return peg.getRadius() <= radius;
    }
}

public class Main {

    public static void main(String[] args) {
        RoundHole hole = new RoundHole(5.0);
        RoundPeg roundPeg = new SimpleRoundPeg(5.0);
        SquarePeg smallSquarePeg = new SquarePeg(5.0);
        SquarePeg largeSquarePeg = new SquarePeg(8.0);

        System.out.println("Round peg fits? " + hole.fits(roundPeg));

        SquarePegAdapter smallAdapter = new SquarePegAdapter(smallSquarePeg);
        System.out.println("Small square peg fits? " + hole.fits(smallAdapter));

        SquarePegAdapter largeAdapter = new SquarePegAdapter(largeSquarePeg);
        System.out.println("Large square peg fits? " + hole.fits(largeAdapter));
    }
}
