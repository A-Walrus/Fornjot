use crate::{
    objects::{Face, Sketch},
    partial2::{Partial, PartialObject},
};

/// A partial [`Sketch`]
///
/// [`Sketch`]: crate::objects::Sketch
pub struct PartialSketch {
    /// The faces that make up the sketch
    pub faces: Vec<Partial<Face>>,
}

impl PartialObject for PartialSketch {
    type Full = Sketch;
}
