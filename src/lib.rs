#![deny(unconditional_recursion)]

pub mod utf8;

pub mod prelude {
    pub use crate::utf8::*;
}
