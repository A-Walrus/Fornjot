//! API for building objects

// These are the old-style builders that need to be transferred to the partial
// object API. Issue:
// https://github.com/hannobraun/Fornjot/issues/1147
mod solid;

// These are new-style builders that build on top of the partial object API.
mod curve;
mod cycle;
mod edge;
mod face;
mod shell;
mod sketch;
mod surface;
mod vertex;

pub use self::{
    curve::CurveBuilder,
    cycle::CycleBuilder,
    edge::{GlobalEdgeBuilder, HalfEdgeBuilder},
    face::FaceBuilder,
    shell::ShellBuilder,
    sketch::SketchBuilder,
    solid::SolidBuilder,
    surface::SurfaceBuilder,
    vertex::{GlobalVertexBuilder, SurfaceVertexBuilder, VertexBuilder},
};
