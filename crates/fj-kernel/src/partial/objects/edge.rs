use crate::{
    builder::GlobalEdgeBuilder,
    objects::{
        Curve, GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge, Objects,
        Surface, Vertex,
    },
    partial::{MaybePartial, MergeWith, PartialCurve, PartialVertex, Replace},
    services::Service,
    storage::Handle,
};

/// A partial [`HalfEdge`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default)]
pub struct PartialHalfEdge {
    /// The vertices that bound the [`HalfEdge`] in the curve
    pub vertices: [MaybePartial<Vertex>; 2],

    /// The global form of the [`HalfEdge`]
    pub global_form: MaybePartial<GlobalEdge>,
}

impl PartialHalfEdge {
    /// Access the partial half-edge's curve
    pub fn curve(&self) -> MaybePartial<Curve> {
        let [a, b] = &self.vertices;
        a.curve().merge_with(b.curve())
    }

    /// Build a full [`HalfEdge`] from the partial half-edge
    pub fn build(self, objects: &mut Service<Objects>) -> HalfEdge {
        let global_curve = self
            .curve()
            .global_form()
            .merge_with(self.global_form.curve());

        let curve = self
            .curve()
            .merge_with(PartialCurve {
                global_form: global_curve,
                ..Default::default()
            })
            .into_full(objects);
        let vertices = self.vertices.map(|vertex| {
            vertex
                .merge_with(PartialVertex {
                    curve: curve.clone().into(),
                    ..Default::default()
                })
                .into_full(objects)
        });

        let global_form = self
            .global_form
            .update_partial(|partial| {
                partial.update_from_curve_and_vertices(&curve, &vertices)
            })
            .into_full(objects);

        HalfEdge::new(vertices, global_form)
    }
}

impl MergeWith for PartialHalfEdge {
    fn merge_with(self, other: impl Into<Self>) -> Self {
        let other = other.into();

        Self {
            vertices: self.vertices.merge_with(other.vertices),
            global_form: self.global_form.merge_with(other.global_form),
        }
    }
}

impl Replace<Surface> for PartialHalfEdge {
    fn replace(&mut self, surface: Handle<Surface>) -> &mut Self {
        for vertex in &mut self.vertices {
            vertex.replace(surface.clone());
        }

        self
    }
}

impl From<&HalfEdge> for PartialHalfEdge {
    fn from(half_edge: &HalfEdge) -> Self {
        let [back_vertex, front_vertex] =
            half_edge.vertices().clone().map(Into::into);

        Self {
            vertices: [back_vertex, front_vertex],
            global_form: half_edge.global_form().clone().into(),
        }
    }
}

/// A partial [`GlobalEdge`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default)]
pub struct PartialGlobalEdge {
    /// The curve that the [`GlobalEdge`] is defined in
    pub curve: MaybePartial<GlobalCurve>,

    /// The vertices that bound the [`GlobalEdge`] in the curve
    pub vertices: [MaybePartial<GlobalVertex>; 2],
}

impl PartialGlobalEdge {
    /// Build a full [`GlobalEdge`] from the partial global edge
    pub fn build(self, objects: &mut Service<Objects>) -> GlobalEdge {
        let curve = self.curve.into_full(objects);
        let vertices = self
            .vertices
            .map(|global_vertex| global_vertex.into_full(objects));

        GlobalEdge::new(curve, vertices)
    }
}

impl MergeWith for PartialGlobalEdge {
    fn merge_with(self, other: impl Into<Self>) -> Self {
        let other = other.into();

        Self {
            curve: self.curve.merge_with(other.curve),
            vertices: self.vertices.merge_with(other.vertices),
        }
    }
}

impl From<&GlobalEdge> for PartialGlobalEdge {
    fn from(global_edge: &GlobalEdge) -> Self {
        Self {
            curve: global_edge.curve().clone().into(),
            vertices: global_edge
                .vertices()
                .access_in_normalized_order()
                .map(Into::into),
        }
    }
}
