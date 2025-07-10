.PHONY: all java rust clean

# Default target
all: java rust

# Run Java adapter example
java:
	@echo "Compiling and running Java adapter example..."
	@javac java_adapter/*.java
	@java -cp java_adapter Main

# Run Rust builder example
rust:
	@echo "Running Rust adapter example..."
	@cd rust_adapter && cargo run

# Clean Java and Rust build artifacts
clean:
	@echo "Cleaning Java class files..."
	@rm -f java_adapter/*.class
	@echo "Cleaning Rust build artifacts..."
	@cd rust_adapter && cargo clean
