use crate::{
    geometry::path::SurfacePath,
    objects::{Curve, GlobalCurve, Objects, Surface},
    partial::{MaybePartial, MergeWith, Replace},
    services::Service,
    storage::Handle,
};

/// A partial [`Curve`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default)]
pub struct PartialCurve {
    /// The path that defines the [`Curve`]
    pub path: Option<SurfacePath>,

    /// The surface that the [`Curve`] is defined in
    pub surface: Option<Handle<Surface>>,

    /// The global form of the [`Curve`]
    pub global_form: MaybePartial<GlobalCurve>,
}

impl PartialCurve {
    /// Build a full [`Curve`] from the partial curve
    pub fn build(self, objects: &mut Service<Objects>) -> Curve {
        let path = self.path.expect("Can't build `Curve` without path");
        let surface =
            self.surface.expect("Can't build `Curve` without surface");

        let global_form = self.global_form.into_full(objects);

        Curve::new(surface, path, global_form)
    }
}

impl MergeWith for PartialCurve {
    fn merge_with(self, other: impl Into<Self>) -> Self {
        let other = other.into();

        Self {
            path: self.path.merge_with(other.path),
            surface: self.surface.merge_with(other.surface),
            global_form: self.global_form.merge_with(other.global_form),
        }
    }
}

impl Replace<Surface> for PartialCurve {
    fn replace(&mut self, surface: Handle<Surface>) -> &mut Self {
        self.surface = Some(surface);
        self
    }
}

impl From<&Curve> for PartialCurve {
    fn from(curve: &Curve) -> Self {
        Self {
            path: Some(curve.path()),
            surface: Some(curve.surface().clone()),
            global_form: curve.global_form().clone().into(),
        }
    }
}

impl MaybePartial<Curve> {
    /// Access the path
    pub fn path(&self) -> Option<SurfacePath> {
        match self {
            Self::Full(full) => Some(full.path()),
            Self::Partial(partial) => partial.path,
        }
    }

    /// Access the surface
    pub fn surface(&self) -> Option<Handle<Surface>> {
        match self {
            Self::Full(full) => Some(full.surface().clone()),
            Self::Partial(partial) => partial.surface.clone(),
        }
    }

    /// Access the global form
    pub fn global_form(&self) -> MaybePartial<GlobalCurve> {
        match self {
            Self::Full(full) => full.global_form().clone().into(),
            Self::Partial(partial) => partial.global_form.clone(),
        }
    }
}

/// A partial [`GlobalCurve`]
///
/// This struct might seem unnecessary. [`GlobalCurve`] literally has nothing in
/// it. Why would we need to represent a part of nothing? However, having this
/// provides some regularity that helps simplify some things within the partial
/// object and builder APIs.
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct PartialGlobalCurve;

impl PartialGlobalCurve {
    /// Build a full [`GlobalCurve`] from the partial global curve
    pub fn build(self, _: &Objects) -> GlobalCurve {
        GlobalCurve
    }
}

impl MergeWith for PartialGlobalCurve {
    fn merge_with(self, _: impl Into<Self>) -> Self {
        Self
    }
}

impl From<&GlobalCurve> for PartialGlobalCurve {
    fn from(_: &GlobalCurve) -> Self {
        Self
    }
}
