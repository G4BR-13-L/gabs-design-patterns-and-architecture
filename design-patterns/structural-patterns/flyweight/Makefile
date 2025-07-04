.PHONY: all java rust clean

all: java rust

java:
	@echo "Compiling and running Java flyweight example..."
	@javac java_flyweight/*.java
	@java -cp java_flyweight Main

rust:
	@echo "Running Rust flyweight example..."
	@cd rust_flyweight && cargo run

clean:
	@echo "Cleaning Java class files..."
	@rm -f java_flyweight/*.class
	@echo "Cleaning Rust build artifacts..."
	@cd rust_flyweight && cargo clean
