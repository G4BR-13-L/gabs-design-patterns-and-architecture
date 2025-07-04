use std::collections::HashMap;
use std::rc::Rc;

struct TreeType {
    name: String,
    color: String,
    texture: String,
}

impl TreeType {
    fn new(name: &str, color: &str, texture: &str) -> Self {
        println!("Creating TreeType: {}", name);
        TreeType {
            name: name.to_string(),
            color: color.to_string(),
            texture: texture.to_string(),
        }
    }

    fn display(&self, x: i32, y: i32) {
        println!(
            "Drawing {} tree at ({}, {}) with color {} and texture {}",
            self.name, x, y, self.color, self.texture
        );
    }
}

struct TreeFactory {
    types: HashMap<String, Rc<TreeType>>,
}

impl TreeFactory {
    fn new() -> Self {
        TreeFactory {
            types: HashMap::new(),
        }
    }

    fn get_tree_type(&mut self, name: &str, color: &str, texture: &str) -> Rc<TreeType> {
        let key = format!("{}:{}:{}", name, color, texture);
        self.types
            .entry(key.clone())
            .or_insert_with(|| Rc::new(TreeType::new(name, color, texture)))
            .clone()
    }
}

struct Tree {
    x: i32,
    y: i32,
    tree_type: Rc<TreeType>,
}

impl Tree {
    fn new(x: i32, y: i32, tree_type: Rc<TreeType>) -> Self {
        Tree { x, y, tree_type }
    }

    fn draw(&self) {
        self.tree_type.display(self.x, self.y)
    }
}

fn main() {
    let mut facttory = TreeFactory::new();

    let oak_type = facttory.get_tree_type("Oak", "Green", "Rough");
    let pine_type = facttory.get_tree_type("Pine", "Dark Green", "Smooth");

    let trees = vec![
        Tree::new(1, 1, oak_type.clone()),
        Tree::new(2, 3, oak_type.clone()),
        Tree::new(4, 5, pine_type.clone()),
    ];

    // With Rust std Rc<T> is a smart pointr to use with multiple immutable data.
    // Its possible to to multiple data owner the same value.
    // Thats the tric of flyweight

    for tree in trees {
        tree.draw();
    }
}
