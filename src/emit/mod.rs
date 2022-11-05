use crate::hit::Hit;

mod json;
mod text;
pub use json::JsonEmitter;
pub use text::TextEmitter;

/// Any emitter (a struct that takes all Hits and emits them in a given format) must implement this
/// trait.
pub trait Emit {
    fn emit(&self, hits: Vec<Hit>);
}
