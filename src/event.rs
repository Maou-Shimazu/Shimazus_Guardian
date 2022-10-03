use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;
type HandlerPtr<T> = Box<dyn Fn(&T)>;

pub struct EventEmitter<T: Hash + Eq, U> {
    handlers: HashMap<T, Vec<HandlerPtr<U>>>,
}

impl<T: Hash + Eq, U> EventEmitter<T, U> {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn on<F>(&mut self, event: T, handler: F)
    where
        F: Fn(&U) + 'static,
    {
        let event_handlers =
            self.handlers.entry(event).or_insert_with(|| vec![]);
        event_handlers.push(Box::new(handler));
    }

    pub fn emit(&self, event: T, payload: U) {
        if let Some(handlers) = self.handlers.get(&event) {
            for handler in handlers {
                handler(&payload);
            }
        }
    }
}