#![deny(unconditional_recursion)]

pub mod ucs2;
pub mod utf8;
pub mod unicode;

pub mod prelude {
    pub use crate::utf8::*;
    pub use crate::ucs2::*;
    pub use crate::unicode::*;
}
