//! API for transforming objects

mod curve;
mod cycle;
mod edge;
mod face;
mod shell;
mod sketch;
mod solid;
mod surface;
mod vertex;

use fj_math::{Transform, Vector};

use crate::{
    insert::Insert,
    objects::Objects,
    partial::{HasPartial, MaybePartial, Partial},
    services::Service,
    storage::Handle,
    validate::{Validate, ValidationError},
};

/// Transform an object
///
/// # Implementation Note
///
/// So far, a general `transform` method is available, along some convenience
/// methods for more specific transformations.
///
/// More convenience methods can be added as required. The only reason this
/// hasn't been done so far, is that no one has put in the work yet.
pub trait TransformObject: Sized {
    /// Transform the object
    fn transform(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
    ) -> Self {
        let mut cache = TransformCache::default();
        self.transform_with_cache(transform, objects, &mut cache)
    }

    /// Transform the object using the provided cache
    fn transform_with_cache(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
        cache: &mut TransformCache,
    ) -> Self;

    /// Translate the object
    ///
    /// Convenience wrapper around [`TransformObject::transform`].
    fn translate(
        self,
        offset: impl Into<Vector<3>>,
        objects: &mut Service<Objects>,
    ) -> Self {
        self.transform(&Transform::translation(offset), objects)
    }

    /// Rotate the object
    ///
    /// Convenience wrapper around [`TransformObject::transform`].
    fn rotate(
        self,
        axis_angle: impl Into<Vector<3>>,
        objects: &mut Service<Objects>,
    ) -> Self {
        self.transform(&Transform::rotation(axis_angle), objects)
    }
}

impl<T> TransformObject for Handle<T>
where
    T: HasPartial + Insert,
    T::Partial: TransformObject,
    ValidationError: From<<T as Validate>::Error>,
{
    fn transform_with_cache(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
        cache: &mut TransformCache,
    ) -> Self {
        self.to_partial()
            .transform_with_cache(transform, objects, cache)
            .build(objects)
            .insert(objects)
    }
}

impl<T> TransformObject for MaybePartial<T>
where
    T: HasPartial,
    Handle<T>: TransformObject,
    T::Partial: TransformObject,
{
    fn transform_with_cache(
        self,
        transform: &Transform,
        objects: &mut Service<Objects>,
        cache: &mut TransformCache,
    ) -> Self {
        let transformed = match self {
            Self::Full(full) => full
                .to_partial()
                .transform_with_cache(transform, objects, cache),
            Self::Partial(partial) => {
                partial.transform_with_cache(transform, objects, cache)
            }
        };

        // Transforming a `MaybePartial` *always* results in a partial object.
        // This provides the most flexibility to the caller, who might want to
        // use the transformed partial object for merging or whatever else,
        // before building it themselves.
        Self::Partial(transformed)
    }
}

/// A cache for transformed objects
///
/// See [`TransformObject`].
#[derive(Default)]
pub struct TransformCache;
