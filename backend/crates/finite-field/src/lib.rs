pub mod gf256;
pub mod gfext;
pub mod gfp;
pub mod field2m;
pub mod primitive;
pub mod prelude {
    pub use crate::error::{FieldError, Result as FieldResult};
}
pub mod error;