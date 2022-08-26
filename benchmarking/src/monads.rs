pub trait Monad {
    type Item;
    fn bind<F: Fn(Self::Item) -> Self>(self, f: F) -> Self;
    fn attempt<F: Fn(Self::Item) -> Self::Item>(self, f: F) -> Self;
}

impl<T> Monad for Option<T> {
    type Item = T;

    fn bind<F: Fn(T) -> Option<T>>(self, f: F) -> Option<T> {
        if let Some(val) = self {
            f(val)
        } else {
            None
        }
    }

    fn attempt<F: Fn(T) -> T>(self, f: F) -> Option<T> {
        if let Some(val) = self {
            Some(f(val))
        } else {
            None
        }
    }
}

/*
pub struct Logger<T> {
    log: Vec<String>,
    outputs: Vec<T>,
}

impl<T> Monad for Logger<T> {
    type Item = T;

    fn bind<F: Fn(T) -> Logger<T>>(self, f: F) -> Logger<T> {
        Logger { log: Vec::new(), outputs: Vec::new()}
    }

    fn attempt<F: Fn(T) -> T>(self, f: F) -> Logger<T> {
        Logger { log: Vec::new(), outputs: Vec::new()}
    }
}
*/