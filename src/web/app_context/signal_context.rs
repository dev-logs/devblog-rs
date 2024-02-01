use std::ops::Deref;
use leptos::{ReadSignal, WriteSignal};

#[derive(Debug, Clone)]
pub struct AppSignalContext<T> where T: Clone + 'static + Default {
    write_signal: WriteSignal<T>,
    read_signal: ReadSignal<T>
}

impl<T> Deref for AppSignalContext<T> where T: Clone + 'static + Default {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl <T> AppSignalContext<T> where T: Clone + 'static + Default {
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

pub trait UseAppSignal<T> where T: Clone + 'static + Default {
    fn attach() -> Self;
    fn read(&self) -> ReadSignal<T>;
}
