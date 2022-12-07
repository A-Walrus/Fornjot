use fj_interop::ext::ArrayExt;
use fj_math::{Point, Scalar};
use iter_fixed::IntoIteratorFixed;

use crate::{
    insert::Insert,
    objects::{Curve, Objects, Surface, Vertex, VerticesInNormalizedOrder},
    partial::{
        MaybePartial, MergeWith, PartialGlobalEdge, PartialHalfEdge,
        PartialSurfaceVertex, PartialVertex,
    },
    partial2::{Partial, PartialCurve},
    services::{Service, Services},
    storage::Handle,
};

use super::CurveBuilder;

/// Builder API for [`PartialHalfEdge`]
pub trait HalfEdgeBuilder: Sized {
    /// Update the partial half-edge with the given back vertex
    fn with_back_vertex(self, back: impl Into<MaybePartial<Vertex>>) -> Self;

    /// Update the partial half-edge with the given front vertex
    fn with_front_vertex(self, front: impl Into<MaybePartial<Vertex>>) -> Self;

    /// Update partial half-edge as a circle, from the given radius
    ///
    /// # Implementation Note
    ///
    /// In principle, only the `build` method should take a reference to
    /// [`Objects`]. As of this writing, this method is the only one that
    /// deviates from that. I couldn't think of a way to do it better.
    fn update_as_circle_from_radius(
        self,
        radius: impl Into<Scalar>,
        objects: &mut Service<Objects>,
    ) -> Self;

    /// Update partial half-edge as a line segment, from the given points
    fn update_as_line_segment_from_points(
        self,
        surface: Partial<Surface>,
        points: [impl Into<Point<2>>; 2],
    ) -> Self;

    /// Update partial half-edge as a line segment, reusing existing vertices
    fn update_as_line_segment(self) -> Self;

    /// Infer the global form of the partial half-edge
    fn infer_global_form(self) -> Self;
}

impl HalfEdgeBuilder for PartialHalfEdge {
    fn with_back_vertex(
        mut self,
        back: impl Into<MaybePartial<Vertex>>,
    ) -> Self {
        let [_, front] = self.vertices.clone();
        self.vertices = [back.into(), front];
        self
    }

    fn with_front_vertex(
        mut self,
        front: impl Into<MaybePartial<Vertex>>,
    ) -> Self {
        let [back, _] = self.vertices.clone();
        self.vertices = [back, front.into()];
        self
    }

    fn update_as_circle_from_radius(
        mut self,
        radius: impl Into<Scalar>,
        objects: &mut Service<Objects>,
    ) -> Self {
        let mut curve = self.curve();
        curve.write().update_as_circle_from_radius(radius);

        let path = curve
            .read()
            .path
            .expect("Expected path that was just created");

        let [a_curve, b_curve] =
            [Scalar::ZERO, Scalar::TAU].map(|coord| Point::from([coord]));

        let [global_vertex, _] = self.global_form.vertices();

        let surface_vertex = PartialSurfaceVertex {
            position: Some(path.point_from_path_coords(a_curve)),
            surface: curve.read().surface.clone(),
            global_form: global_vertex,
        }
        .build(objects)
        .insert(objects);

        let [back, front] =
            [a_curve, b_curve].map(|point_curve| PartialVertex {
                position: Some(point_curve),
                curve: curve.clone(),
                surface_form: surface_vertex.clone().into(),
            });

        self.vertices = [back, front].map(Into::into);

        self
    }

    fn update_as_line_segment_from_points(
        mut self,
        surface: Partial<Surface>,
        points: [impl Into<Point<2>>; 2],
    ) -> Self {
        self.vertices = self.vertices.zip_ext(points).map(|(vertex, point)| {
            let mut vertex = vertex.into_partial();

            vertex.curve = {
                let curve = vertex.curve.read().clone();
                Partial::from_partial(PartialCurve {
                    surface: surface.clone(),
                    ..curve
                })
            };
            vertex.surface_form = MaybePartial::from(PartialSurfaceVertex {
                position: Some(point.into()),
                surface: surface.clone(),
                ..Default::default()
            });

            vertex.into()
        });

        self.update_as_line_segment()
    }

    fn update_as_line_segment(mut self) -> Self {
        let [from, to] = self.vertices.clone();
        let [from_surface, to_surface] =
            [&from, &to].map(|vertex| vertex.surface_form());

        let surface = self.curve().read().surface.clone();
        let points = [&from_surface, &to_surface].map(|vertex| {
            vertex
                .position()
                .expect("Can't infer line segment without surface position")
        });

        let mut curve = self.curve();
        curve.write().surface = surface;
        curve.write().update_as_line_from_points(points);

        let [back, front] = {
            let vertices = [(from, 0.), (to, 1.)].map(|(vertex, position)| {
                vertex.update_partial(|mut vertex| {
                    vertex.position = Some([position].into());
                    vertex.curve = self.curve();
                    vertex
                })
            });

            // The global vertices we extracted are in normalized order, which
            // means we might need to switch their order here. This is a bit of
            // a hack, but I can't think of something better.
            let global_forms = {
                let must_switch_order = {
                    let mut services = Services::new();
                    let vertices = vertices.clone().map(|vertex| {
                        vertex
                            .into_full(&mut services.objects)
                            .global_form()
                            .clone()
                    });

                    let (_, must_switch_order) =
                        VerticesInNormalizedOrder::new(vertices);

                    must_switch_order
                };

                let [a, b] = self.global_form.vertices();
                if must_switch_order {
                    [b, a]
                } else {
                    [a, b]
                }
            };

            vertices
                .into_iter_fixed()
                .zip(global_forms)
                .collect::<[_; 2]>()
                .map(|(vertex, global_form)| {
                    vertex.update_partial(|mut vertex| {
                        vertex.surface_form = vertex.surface_form.merge_with(
                            PartialSurfaceVertex {
                                global_form,
                                ..Default::default()
                            },
                        );
                        vertex
                    })
                })
        };

        self.vertices = [back, front];

        self
    }

    fn infer_global_form(mut self) -> Self {
        self.global_form = PartialGlobalEdge::default().into();
        self
    }
}

/// Builder API for [`PartialGlobalEdge`]
pub trait GlobalEdgeBuilder {
    /// Update partial global edge from the given curve and vertices
    fn update_from_curve_and_vertices(
        self,
        curve: &Curve,
        vertices: &[Handle<Vertex>; 2],
    ) -> Self;
}

impl GlobalEdgeBuilder for PartialGlobalEdge {
    fn update_from_curve_and_vertices(
        mut self,
        curve: &Curve,
        vertices: &[Handle<Vertex>; 2],
    ) -> Self {
        self.curve =
            Partial::from_full_entry_point(curve.global_form().clone());
        self.vertices = vertices
            .clone()
            .map(|vertex| vertex.global_form().clone().into());
        self
    }
}
