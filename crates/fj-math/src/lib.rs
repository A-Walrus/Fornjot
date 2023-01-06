//! # Fornjot Math Library
//!
//! This library is part of the [Fornjot] ecosystem. Fornjot is an open-source,
//! code-first CAD application; and collection of libraries that make up the CAD
//! application, but can be used independently.
//!
//! This library is an internal component of Fornjot. It is not relevant to end
//! users that just want to create CAD models.
//!
//! This crates provides basic math types for the Fornjot ecosystem. It is built
//! on [nalgebra] and [Parry], but provides an interface that is specifically
//! tailored to the needs of Fornjot.
//!
//! ## Failing [`From`]/[`Into`] implementations
//!
//! Please note that any [`From`]/[`Into`] implementation that convert floating
//! point numbers into [`Scalar`] can panic. These conversions call
//! [`Scalar::from_f64`] internally and panic under the same conditions. This
//! affects [`Scalar`] itself, but also any other types in this crate that
//! provide conversions from types that involve `f64`.
//!
//! This explicitly goes against the mandate of [`From`]/[`Into`], whose
//! documentation states that implementations must not fail. This is a
//! deliberate design decision. The intended use case of `Scalar` is math code
//! that considers NaN results a bug, not a recoverable error.
//!
//! For this use case, having easy conversions available is an advantage, and
//! explicit `unwrap`/`expect` calls would add nothing. In addition, the
//! [`From`]/[`Into`] documentation fails to provide any reasons for its
//! mandate.
//!
//! [Fornjot]: https://www.fornjot.app/
//! [nalgebra]: https://nalgebra.org/
//! [Parry]: https://www.parry.rs/

#![warn(missing_docs)]

pub mod robust;

mod aabb;
mod arc;
mod circle;
mod coordinates;
mod line;
mod plane;
mod point;
mod poly_chain;
mod scalar;
mod segment;
mod transform;
mod triangle;
mod vector;

pub use self::{
    aabb::Aabb,
    arc::Arc,
    circle::Circle,
    coordinates::{Uv, Xyz, T},
    line::Line,
    plane::Plane,
    point::Point,
    poly_chain::PolyChain,
    scalar::{Scalar, Sign},
    segment::Segment,
    transform::Transform,
    triangle::{Triangle, Winding},
    vector::Vector,
};
