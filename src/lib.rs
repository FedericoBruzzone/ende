#![deny(unconditional_recursion)]

pub mod ucs2;
pub mod unicode;
pub mod utf16;
pub mod utf8;

pub mod prelude {
    pub use crate::ucs2::*;
    pub use crate::unicode::*;
    pub use crate::utf16::*;
    pub use crate::utf8::*;
}
