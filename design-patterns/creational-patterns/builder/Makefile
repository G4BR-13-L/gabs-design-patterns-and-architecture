.PHONY: all java rust clean

# Default target
all: java rust

# Run Java strategy example
java:
	@echo "Compiling and running Java builder example..."
	@javac java_builder/*.java
	@java -cp java_builder User

# Run Rust strategy example
rust:
	@echo "Running Rust builder example..."
	@cd rust_builder && cargo run

# Clean Java and Rust build artifacts
clean:
	@echo "Cleaning Java class files..."
	@rm -f java_builder/*.class
	@echo "Cleaning Rust build artifacts..."
	@cd rust_builder && cargo clean
