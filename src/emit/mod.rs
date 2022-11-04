use crate::hit::Hit;

mod json;
mod text;
pub use json::JsonEmitter;
pub use text::TextEmitter;

pub trait Emit {
    fn emit(&self, hits: Vec<Hit>);
}
