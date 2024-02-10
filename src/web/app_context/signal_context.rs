use leptos::{create_signal, provide_context, ReadSignal, use_context, WriteSignal};

#[derive(Debug, Clone)]
pub struct AppSignal<T> where T: Clone + 'static + Default {
    write_signal: WriteSignal<T>,
    read_signal: ReadSignal<T>
}

impl <T> AppSignal<T> where T: Clone + 'static + Default {
    pub fn new (read: ReadSignal<T>, write: WriteSignal<T>) -> Self {
        Self {
            read_signal: read,
            write_signal: write
        }
    }

    pub fn write(&self) -> WriteSignal<T> {
        self.write_signal.clone()
    }

    pub fn read(&self) -> ReadSignal<T> {
        self.read_signal.clone()
    }
}

pub trait AppContextProvider {
    fn attach();
}

pub trait AppContext {
    fn new () -> Self;
}

impl<T> AppContextProvider for T where T: Clone + AppContext + 'static {
    fn attach() {
        if let None = use_context::<Self>() {
            provide_context(Self::new())
        }
    }
}
