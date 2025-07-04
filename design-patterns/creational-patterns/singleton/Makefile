.PHONY: all java rust clean

# Default target
all: java rust

# Run Java singleton example
java:
	@echo "Compiling and running Java singleton example..."
	@javac java_singleton/*.java
	@java -cp java_singleton Logger

# Run Rust builder example
rust:
	@echo "Running Rust singleton example..."
	@cd rust_singleton && cargo run

# Clean Java and Rust build artifacts
clean:
	@echo "Cleaning Java class files..."
	@rm -f java_singleton/*.class
	@echo "Cleaning Rust build artifacts..."
	@cd rust_singleton && cargo clean
