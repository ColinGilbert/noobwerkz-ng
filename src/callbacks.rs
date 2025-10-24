use std::{future::Future, pin::Pin};

pub trait AsyncCallback<T> {
    type Output: Future<Output = ()>; // Or a specific return type
    fn call(&self, arg: T) -> Self::Output;
}

impl<T, F, Fut> AsyncCallback<T> for F
where
    F: Fn(T) -> Fut,
    Fut: Future<Output = ()>, // Match the Output of the trait
{
    type Output = Fut;
    fn call(&self, arg: T) -> Fut {
        self(arg)
    }
}