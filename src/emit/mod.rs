use crate::hit::Hit;

mod text;
pub use text::TextEmitter;

pub trait Emit {
    fn emit(&self, hits: Vec<Hit>);
}
