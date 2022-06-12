use crate::if_unlikely;
use std::intrinsics::unlikely;
use std::marker::Unsize;

#[cfg(feature = "log")]
#[derive(Default)]
pub struct Logger {
    inner: Option<Box<dyn InnerLogger>>,
}

#[cfg(not(feature = "log"))]
#[derive(Default, Clone)]
pub struct Logger {}

impl Logger {
    #[inline]
    pub fn none() -> Self {
        Self::default()
    }
}

#[cfg(feature = "log")]
impl Logger {
    #[inline]
    pub fn printer(get_title: impl FnOnce() -> String) -> Self {
        Self {
            inner: Some(Box::new(Printer {
                title: get_title(),
            })),
        }
    }

    #[inline]
    pub fn print<S: Into<String> + Sized, F: FnOnce() -> S>(&self, f: F) {
        if_unlikely!(let Some(x) = &self.inner => {
            x.log(f().into());
        })
    }
}

#[cfg(not(feature = "log"))]
impl Logger {
    #[inline]
    pub fn printer(title: impl FnOnce() -> String) -> Self {
        Logger {}
    }

    #[inline]
    pub fn print<S: Into<String> + Sized, F: FnOnce() -> S>(&self, f: F) {}
}

#[cfg(feature = "log")]
impl Clone for Logger {
    #[inline]
    fn clone(&self) -> Self {
        Logger{
            inner: self.inner.as_ref().map(|x| x.split())
        }
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
    #[inline]
    fn log(&self, s: String) {
        println!("<{title}> {s}", title = self.title, s = s);
    }

    #[inline]
    fn split(&self) -> Box<dyn InnerLogger> {
        Box::new(Printer {
            title: self.title.clone(),
        })
    }
}
