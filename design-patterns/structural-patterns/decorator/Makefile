.PHONY: all java rust clean

# Default target
all: java rust

# Run Java decorator example
java:
	@echo "Compiling and running Java decorator example..."
	@javac java_decorator/*.java
	@java -cp java_decorator Main

# Run Rust decorator example
rust:
	@echo "Running Rust decorator example..."
	@cd rust_decorator && cargo run

# Clean Java and Rust build artifacts
clean:
	@echo "Cleaning Java class files..."
	@rm -f java_builder/*.class
	@echo "Cleaning Rust build artifacts..."
	@cd rust_decorator && cargo clean
