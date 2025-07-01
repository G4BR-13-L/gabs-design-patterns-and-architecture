.PHONY: all java rust clean

# Default target
all: java rust

# Run Java strategy example
java:
	@echo "Compiling and running Java strategy example..."
	@javac java_strategy/*.java
	@java -cp java_strategy Main

# Run Rust strategy example
rust:
	@echo "Running Rust strategy example..."
	@cd rust_strategy && cargo run

# Clean Java and Rust build artifacts
clean:
	@echo "Cleaning Java class files..."
	@rm -f java_strategy/*.class
	@echo "Cleaning Rust build artifacts..."
	@cd rust_strategy && cargo clean
