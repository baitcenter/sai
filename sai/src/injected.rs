use std::sync::{Arc};
use std::ops::Deref;
use std::boxed::Box;
use std::any::Any;
use super::downcast::Downcast;

/// The official wrapper for all dependencies.
///
/// This is Essentially a a wrapper for Arc.
#[derive(Default)]
pub struct Injected<T: ?Sized> {
    item: Arc<T>,
}

impl<T> Injected<T> {
    pub fn new(val: T) -> Injected<T> {
        return Injected {
            item: Arc::new(val),
        }
    }
}

impl<T: Downcast + 'static + ?Sized> Injected<T> {
    pub fn downcast<S: Any + Send + Sync>(self) -> Option<Injected<S>> {
        return Some(Injected {
            item: self.item.into_any_arc().downcast().unwrap(),
        })
    }
}

impl<T: ?Sized> Injected<T> {

    fn from_arc(val: Arc<T>) -> Injected<T> {
        return Injected {
            item: val
        }
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        let v = &mut self.item;
        Arc::get_mut(v)
    }
}

impl<T: ?Sized> Clone for Injected<T> {
    fn clone(&self) -> Injected<T> {
        return Injected {
            item: self.item.clone()
        }
    }
}

impl<T: ?Sized> From<Box<T>> for Injected<T> {
    fn from(m: Box<T>) -> Self {
        let arc: Arc<T> = m.into();
        return Injected::from_arc(arc);
    }
}

impl<T> Deref for Injected<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Arc<T> {
        &self.item
    }
}
