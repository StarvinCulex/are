use crate::if_unlikely;
use std::intrinsics::unlikely;
use std::marker::Unsize;

#[derive(Default)]
pub struct Logger {
    inner: Option<Box<dyn InnerLogger>>,
}

impl Logger {
    pub fn none() -> Self {
        Self::default()
    }
    pub fn printer(title: String) -> Self {
        Logger {
            inner: Some(Box::new(Printer { title })),
        }
    }

    #[inline]
    pub fn print<S: Into<String> + Sized, F: FnOnce() -> S>(&self, f: F) {
        if_unlikely!(let Some(x) = &self.inner => {
            x.log(f().into());
        })
    }
}

impl Clone for Logger {
    fn clone(&self) -> Self {
        if_unlikely!(let Some(x) = &self.inner => {
            Logger{
                inner: Some(x.split())
            }
        } else {
            Logger {
                inner: None
            }
        })
    }
}

trait InnerLogger: Send + Sync {
    fn log(&self, s: String);

    fn split(&self) -> Box<dyn InnerLogger>;
}

struct Printer {
    title: String,
}

impl InnerLogger for Printer {
    fn log(&self, s: String) {
        println!("<{title}> {s}", title = self.title, s = s);
    }

    fn split(&self) -> Box<dyn InnerLogger> {
        Box::new(Printer {
            title: self.title.clone(),
        })
    }
}
