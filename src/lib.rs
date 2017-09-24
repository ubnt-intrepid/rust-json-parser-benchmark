#![feature(fn_must_use)]

#[cfg_attr(test, macro_use)]
extern crate json;
extern crate pikkr;
extern crate serde;
#[cfg_attr(test, macro_use)]
extern crate serde_json;
extern crate mison;

mod executor;
mod targets;

pub use executor::Executor;
