#![allow(unstable)]
extern crate delivery;
#[macro_use] extern crate log;

mod support;

// Thanks, Cargo.
macro_rules! test {
    ($name:ident $expr:expr) => (
        #[test]
        fn $name() {
            setup();
            $expr;
        }
    )
}

mod config;
