use once_cell::sync::Lazy;
use std::sync::Mutex;

struct Logger {
    level: String,
}

impl Logger {
    fn log(&self, message: &str) {
        println!("[{}] {}", self.level, message);
    }

    fn set_level(&mut self, level: &str) {
        self.level = level.to_string();
    }
}

static LOGGER: Lazy<Mutex<Logger>> = Lazy::new(|| {
    Mutex::new(Logger {
        level: "INFO".to_string(),
    })
});

fn main() {
    {
        let mut logger = LOGGER.lock().unwrap();
        logger.log("Starting application...");
        logger.set_level("DEBUG");
    }

    {
        let loggger = LOGGER.lock().unwrap();
        loggger.log("This is a debug message.");
    }
}