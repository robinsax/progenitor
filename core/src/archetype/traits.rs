extern crate proc_macro;

use serde::{Deserialize, Serialize};

pub trait Mutation<T> {
    fn apply(&self, target: &T) -> T;
}

pub trait Stored<'a>: Deserialize<'a> + Serialize
    where Self: std::marker::Sized {
}
