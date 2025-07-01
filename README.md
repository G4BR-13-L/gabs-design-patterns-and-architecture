# 🎯 Gabs' Design Patterns & Architecture Playground

> ✨ _"A software engineer's journey through design patterns in Java & Rust, on the road to international mastery."_

---

## 🧠 Purpose

This repository was created as a **practical lab** to implement and study classic **design patterns** and **software architecture principles**. The goal is to build deep understanding by recreating patterns in **multiple languages**, primarily **Rust 🦀** and **Java ☕**, preparing for a robust international career in software engineering.

Whether you're studying for interviews, building clean systems, or just learning how to write better software—this repo is a hands-on source of inspiration and learning!

---

## 🗂️ Project Structure

```bash
design-patterns/
├── behavioral-patterns/
│   └── strategy/             # Strategy Pattern Implementations
│       ├── java_strategy/    # Java version of Strategy
│       ├── rust_strategy/    # Rust version of Strategy
│       └── Makefile          # Easy runner for both implementations
├── creational-patterns/      # 🚧 Work in progress
├── structural-patterns/      # 🚧 Work in progress
````

Each pattern will have:

* ✅ One or more implementations in **Rust** and **Java**
* 📄 Simple entry points (`main.rs`, `Main.java`)
* 🧪 Self-contained examples for learning and testing
* 🛠️ A `Makefile` to build and run examples easily

---

## 🚀 Getting Started

### Prerequisites

* [Java 17+](https://openjdk.org/) or later
* [Rust (via rustup)](https://www.rust-lang.org/tools/install)
* GNU Make (`make`)

### Running the examples

```bash
cd design-patterns/behavioural-patterns/strategy
make        # Runs both Java and Rust implementations
make java   # Only Java
make rust   # Only Rust
make clean  # Clean build artifacts
```

---

## 📌 Roadmap

* [x] Strategy Pattern (Java + Rust)
* [ ] Observer Pattern
* [ ] Builder Pattern
* [ ] Factory Method
* [ ] Adapter, Bridge, Decorator...
* [ ] Clean Architecture & DDD demos
* [ ] Real-world applications & microservices (Rust + Spring Boot)

---

## 💡 About Me

I'm Gabriel Victor – a Brazilian 🇧🇷 software engineer passionate about clean code, backend architecture, and building a solid international career. I currently work primarily with **Java + Spring**, but I'm leveling up fast in **Rust**, aiming to build robust and high-performance systems.

> Follow the journey. Learn. Build. Grow. 🚀

---

## 📄 License

[MIT](./LICENSE) – Feel free to fork and contribute!
