import java.util.*;

// Intrinsic state
class TreeType {

    private final String name;
    private final String color;
    private final String texture;

    public TreeType(String name, String color, String texture) {
        System.out.println("Creating TreeType: " + name);
        this.name = name;
        this.color = color;
        this.texture = texture;
    }

    public void display(int x, int y) {
        System.out.printf(
            "Drawing %s tree at (%d, %d) with color %s and texture %s%n",
            name,
            x,
            y,
            color,
            texture
        );
    }
}

// Flyweight factory
class TreeFactory {

    private static final Map<String, TreeType> types = new HashMap<>();

    public static TreeType getTreeType(
        String name,
        String color,
        String texture
    ) {
        String key = name + ":" + color + ":" + texture;
        types.putIfAbsent(key, new TreeType(name, color, texture));
        return types.get(key);
    }
}

// Context
class Tree {

    private final int x, y;
    private final TreeType type;

    public Tree(int x, int y, TreeType type) {
        this.x = x;
        this.y = y;
        this.type = type;
    }

    public void draw() {
        type.display(x, y);
    }
}

public class Main {

    public static void main(String[] args) {
        List<Tree> trees = new ArrayList<>();
        trees.add(
            new Tree(1, 1, TreeFactory.getTreeType("Oak", "Green", "Rough"))
        );
        trees.add(
            new Tree(2, 3, TreeFactory.getTreeType("Oak", "Green", "Rough"))
        );
        trees.add(
            new Tree(
                4,
                5,
                TreeFactory.getTreeType("Pine", "Dark Green", "Smooth")
            )
        );

        for (Tree tree : trees) {
            tree.draw();
        }
    }
}
