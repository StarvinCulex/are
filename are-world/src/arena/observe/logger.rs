#[derive(Default)]
pub struct Logger {
    inner: Option<Box<dyn InnerLogger>>,
}

impl Logger {
    pub fn none() -> Self {
        Self::default()
    }
    pub fn printer() -> Self {
        Logger {
            inner: Some(Box::new(Printer {})),
        }
    }
}

trait InnerLogger {
    fn log(&self, s: String);
}

pub struct Printer {}

impl InnerLogger for Printer {
    fn log(&self, s: String) {
        println!("{}", s);
    }
}
