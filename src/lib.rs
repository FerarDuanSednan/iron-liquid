extern crate iron;
extern crate liquid;
extern crate plugin;
extern crate rustc_serialize;

#[macro_use]
extern crate log;

pub use middleware::{LiquidEngine, Template};

mod middleware;

