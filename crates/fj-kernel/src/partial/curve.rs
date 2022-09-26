use fj_math::{Point, Scalar, Vector};

use crate::{
    objects::{Curve, GlobalCurve, Surface},
    path::{GlobalPath, SurfacePath},
    stores::{Handle, Stores},
};

/// A partial [`Curve`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct PartialCurve {
    /// The path that defines the [`Curve`]
    ///
    /// Must be provided before calling [`PartialCurve::build`].
    pub path: Option<SurfacePath>,

    /// The surface that the [`Curve`] is defined in
    ///
    /// Must be provided before calling [`PartialCurve::build`].
    pub surface: Option<Surface>,

    /// The global form of the [`Curve`]
    ///
    /// Will be computed from `path` and `surface` in [`PartialCurve::build`],
    /// if not provided.
    pub global_form: Option<Handle<GlobalCurve>>,
}

impl PartialCurve {
    /// Provide a path for the partial curve
    pub fn with_path(mut self, path: SurfacePath) -> Self {
        self.path = Some(path);
        self
    }

    /// Provide a surface for the partial curve
    pub fn with_surface(mut self, surface: Surface) -> Self {
        self.surface = Some(surface);
        self
    }

    /// Provide a global form for the partial curve
    pub fn with_global_form(
        mut self,
        global_form: Handle<GlobalCurve>,
    ) -> Self {
        self.global_form = Some(global_form);
        self
    }

    /// Update partial curve to represent the u-axis
    pub fn as_u_axis(self) -> Self {
        let a = Point::origin();
        let b = a + Vector::unit_u();

        self.as_line_from_points([a, b])
    }

    /// Update partial curve to represent the v-axis
    pub fn as_v_axis(self) -> Self {
        let a = Point::origin();
        let b = a + Vector::unit_v();

        self.as_line_from_points([a, b])
    }

    /// Update partial curve as a circle, from the provided radius
    pub fn as_circle_from_radius(self, radius: impl Into<Scalar>) -> Self {
        self.with_path(SurfacePath::circle_from_radius(radius))
    }

    /// Update partial curve as a line, from the provided points
    pub fn as_line_from_points(self, points: [impl Into<Point<2>>; 2]) -> Self {
        self.with_path(SurfacePath::line_from_points(points))
    }

    /// Build a full [`Curve`] from the partial curve
    pub fn build(self, stores: &Stores) -> Curve {
        let path = self.path.expect("Can't build `Curve` without path");
        let surface =
            self.surface.expect("Can't build `Curve` without surface");

        let global_form = self.global_form.unwrap_or_else(|| {
            let path = match path {
                SurfacePath::Circle(circle) => {
                    GlobalPath::circle_from_radius(circle.radius())
                }
                SurfacePath::Line(line) => GlobalPath::line_from_points(
                    [line.origin(), line.origin() + line.direction()]
                        .map(|point| surface.point_from_surface_coords(point)),
                ),
            };

            GlobalCurve::partial().with_path(path).build(stores)
        });

        Curve::new(surface, path, global_form)
    }
}

impl From<Curve> for PartialCurve {
    fn from(curve: Curve) -> Self {
        Self {
            path: Some(curve.path()),
            surface: Some(*curve.surface()),
            global_form: Some(curve.global_form().clone()),
        }
    }
}

/// A partial [`GlobalCurve`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct PartialGlobalCurve {
    /// The path that defines the [`GlobalCurve`]
    ///
    /// Must be provided before [`PartialGlobalCurve::build`] is called.
    pub path: Option<GlobalPath>,
}

impl PartialGlobalCurve {
    /// Provide a path for the partial global curve
    pub fn with_path(mut self, path: GlobalPath) -> Self {
        self.path = Some(path);
        self
    }

    /// Update partial global curve to represent the x-axis
    pub fn as_x_axis(self) -> Self {
        self.with_path(GlobalPath::x_axis())
    }

    /// Update partial global curve to represent the y-axis
    pub fn as_y_axis(self) -> Self {
        self.with_path(GlobalPath::y_axis())
    }

    /// Update partial global curve to represent the z-axis
    pub fn as_z_axis(self) -> Self {
        self.with_path(GlobalPath::z_axis())
    }

    /// Update partial global curve as a circle, from the provided radius
    pub fn as_circle_from_radius(self, radius: impl Into<Scalar>) -> Self {
        self.with_path(GlobalPath::circle_from_radius(radius))
    }

    /// Update partial global curve as a line, from the provided points
    pub fn as_line_from_points(self, points: [impl Into<Point<3>>; 2]) -> Self {
        self.with_path(GlobalPath::line_from_points(points))
    }

    /// Build a full [`GlobalCurve`] from the partial global curve
    ///
    /// # Panics
    ///
    /// Panics, if no path was provided.
    pub fn build(self, stores: &Stores) -> Handle<GlobalCurve> {
        let path = self.path.expect("Can't build `GlobalCurve` without a path");
        stores.global_curves.insert(GlobalCurve::from_path(path))
    }
}

impl From<Handle<GlobalCurve>> for PartialGlobalCurve {
    fn from(global_curve: Handle<GlobalCurve>) -> Self {
        Self {
            path: Some(global_curve.path()),
        }
    }
}