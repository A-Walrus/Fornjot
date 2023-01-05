use std::convert::Infallible;

use fj_interop::ext::ArrayExt;
use fj_math::{Point, Scalar};

use crate::{
    objects::{
        Curve, GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge,
        VerticesInNormalizedOrder,
    },
    storage::Handle,
};

use super::{Validate, ValidationConfig};

impl Validate for HalfEdge {
    type Error = HalfEdgeValidationError;

    fn validate_with_config(
        &self,
        config: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        HalfEdgeValidationError::check_curve_identity(self)?;
        HalfEdgeValidationError::check_global_curve_identity(self)?;
        HalfEdgeValidationError::check_global_vertex_identity(self)?;
        HalfEdgeValidationError::check_vertex_positions(self, config)?;

        // We don't need to check anything about surfaces here. We already check
        // curves, which makes sure the vertices are consistent with each other,
        // and the validation of those vertices checks the surfaces.

        Ok(())
    }
}

impl Validate for GlobalEdge {
    type Error = Infallible;

    fn validate_with_config(
        &self,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// [`HalfEdge`] validation failed
#[derive(Clone, Debug, thiserror::Error)]
pub enum HalfEdgeValidationError {
    /// [`HalfEdge`] vertices are not defined on the same `Curve`
    #[error(
        "`HalfEdge` vertices are not defined on the same `Curve`\n\
        - `Curve` of back vertex: {back_curve:#?}\n\
        - `Curve` of front vertex: {front_curve:#?}"
    )]
    CurveMismatch {
        /// The curve of the [`HalfEdge`]'s back vertex
        back_curve: Handle<Curve>,

        /// The curve of the [`HalfEdge`]'s front vertex
        front_curve: Handle<Curve>,
    },

    /// [`HalfEdge`]'s [`GlobalCurve`]s do not match
    #[error(
        "Global form of `HalfEdge`'s `Curve` does not match `GlobalCurve` of \n\
        the `HalfEdge`'s `GlobalEdge`\n\
        - `GlobalCurve` from `Curve`: {global_curve_from_curve:#?}\n\
        - `GlobalCurve` from `GlobalEdge`: {global_curve_from_global_form:#?}",
    )]
    GlobalCurveMismatch {
        /// The [`GlobalCurve`] from the [`HalfEdge`]'s [`Curve`]
        global_curve_from_curve: Handle<GlobalCurve>,

        /// The [`GlobalCurve`] from the [`HalfEdge`]'s global form
        global_curve_from_global_form: Handle<GlobalCurve>,
    },

    /// [`HalfEdge`]'s [`GlobalVertex`] objects do not match
    #[error(
        "Global forms of `HalfEdge` vertices do not match vertices of \n\
        `HalfEdge`'s global form\n\
        - `GlobalVertex` objects from `Vertex` objects: \
            {global_vertices_from_vertices:#?}\n\
        - `GlobalVertex` objects from `GlobalEdge`: \
            {global_vertices_from_global_form:#?}"
    )]
    GlobalVertexMismatch {
        /// The [`GlobalVertex`] from the [`HalfEdge`]'s vertices
        global_vertices_from_vertices: [Handle<GlobalVertex>; 2],

        /// The [`GlobalCurve`] from the [`HalfEdge`]'s global form
        global_vertices_from_global_form: [Handle<GlobalVertex>; 2],
    },

    /// [`HalfEdge`]'s vertices are coincident
    #[error(
        "Vertices on curve are coincident\n\
        - Position of back vertex: {back_position:?}\n\
        - Position of front vertex: {front_position:?}"
    )]
    VerticesAreCoincident {
        /// The position of the back vertex
        back_position: Point<1>,

        /// The position of the front vertex
        front_position: Point<1>,

        /// The distance between the two vertices
        distance: Scalar,
    },
}

impl HalfEdgeValidationError {
    fn check_curve_identity(half_edge: &HalfEdge) -> Result<(), Self> {
        let back_curve = half_edge.back().curve();
        let front_curve = half_edge.front().curve();

        if back_curve.id() != front_curve.id() {
            return Err(Self::CurveMismatch {
                back_curve: back_curve.clone(),
                front_curve: front_curve.clone(),
            });
        }

        Ok(())
    }

    fn check_global_curve_identity(half_edge: &HalfEdge) -> Result<(), Self> {
        let global_curve_from_curve = half_edge.curve().global_form();
        let global_curve_from_global_form = half_edge.global_form().curve();

        if global_curve_from_curve.id() != global_curve_from_global_form.id() {
            return Err(Self::GlobalCurveMismatch {
                global_curve_from_curve: global_curve_from_curve.clone(),
                global_curve_from_global_form: global_curve_from_global_form
                    .clone(),
            });
        }

        Ok(())
    }

    fn check_global_vertex_identity(half_edge: &HalfEdge) -> Result<(), Self> {
        let global_vertices_from_vertices = {
            let (global_vertices_from_vertices, _) =
                VerticesInNormalizedOrder::new(
                    half_edge.vertices().each_ref_ext().map(|vertex| {
                        vertex.surface_form().global_form().clone()
                    }),
                );

            global_vertices_from_vertices.access_in_normalized_order()
        };
        let global_vertices_from_global_form = half_edge
            .global_form()
            .vertices()
            .access_in_normalized_order();

        let ids_from_vertices = global_vertices_from_vertices
            .each_ref_ext()
            .map(|global_vertex| global_vertex.id());
        let ids_from_global_form = global_vertices_from_global_form
            .each_ref_ext()
            .map(|global_vertex| global_vertex.id());

        if ids_from_vertices != ids_from_global_form {
            return Err(Self::GlobalVertexMismatch {
                global_vertices_from_vertices,
                global_vertices_from_global_form,
            });
        }

        Ok(())
    }

    fn check_vertex_positions(
        half_edge: &HalfEdge,
        config: &ValidationConfig,
    ) -> Result<(), Self> {
        let back_position = half_edge.back().position();
        let front_position = half_edge.front().position();

        let distance = (back_position - front_position).magnitude();

        if distance < config.distinct_min_distance {
            return Err(Self::VerticesAreCoincident {
                back_position,
                front_position,
                distance,
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        builder::HalfEdgeBuilder,
        insert::Insert,
        objects::{GlobalCurve, HalfEdge},
        partial::{
            Partial, PartialHalfEdge, PartialObject, PartialSurfaceVertex,
            PartialVertex,
        },
        services::Services,
        validate::Validate,
    };

    #[test]
    fn half_edge_curve_mismatch() {
        let mut services = Services::new();

        let valid = {
            let mut half_edge = PartialHalfEdge::default();
            half_edge.update_as_line_segment_from_points(
                services.objects.surfaces.xy_plane(),
                [[0., 0.], [1., 0.]],
            );

            half_edge.build(&mut services.objects)
        };
        let invalid = {
            let mut vertices = valid.vertices().clone();
            let mut vertex = Partial::from(vertices[1].clone());
            // Arranging for an equal but not identical curve here.
            vertex.write().curve = Partial::from_partial(
                Partial::from(valid.curve().clone()).read().clone(),
            );
            vertices[1] = vertex.build(&mut services.objects);

            HalfEdge::new(vertices, valid.global_form().clone())
        };

        assert!(valid.validate().is_ok());
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn half_edge_global_curve_mismatch() {
        let mut services = Services::new();

        let valid = {
            let mut half_edge = PartialHalfEdge::default();
            half_edge.update_as_line_segment_from_points(
                services.objects.surfaces.xy_plane(),
                [[0., 0.], [1., 0.]],
            );

            half_edge.build(&mut services.objects)
        };
        let invalid = HalfEdge::new(valid.vertices().clone(), {
            let mut tmp = Partial::from(valid.global_form().clone());
            tmp.write().curve =
                Partial::from(GlobalCurve.insert(&mut services.objects));
            tmp.build(&mut services.objects)
        });

        assert!(valid.validate().is_ok());
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn half_edge_global_vertex_mismatch() {
        let mut services = Services::new();

        let valid = {
            let mut half_edge = PartialHalfEdge::default();
            half_edge.update_as_line_segment_from_points(
                services.objects.surfaces.xy_plane(),
                [[0., 0.], [1., 0.]],
            );

            half_edge.build(&mut services.objects)
        };
        let invalid = HalfEdge::new(valid.vertices().clone(), {
            let mut tmp = Partial::from(valid.global_form().clone());
            tmp.write().vertices = valid
                .global_form()
                .vertices()
                .access_in_normalized_order()
                // Creating equal but not identical vertices here.
                .map(|vertex| {
                    Partial::from_partial(Partial::from(vertex).read().clone())
                });
            tmp.build(&mut services.objects)
        });

        assert!(valid.validate().is_ok());
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn half_edge_vertices_are_coincident() {
        let mut services = Services::new();

        let valid = {
            let mut half_edge = PartialHalfEdge::default();
            half_edge.update_as_line_segment_from_points(
                services.objects.surfaces.xy_plane(),
                [[0., 0.], [1., 0.]],
            );

            half_edge.build(&mut services.objects)
        };
        let invalid = HalfEdge::new(
            valid.vertices().clone().map(|vertex| {
                let vertex = Partial::from(vertex).read().clone();
                let surface = vertex.surface_form.read().surface.clone();
                PartialVertex {
                    position: Some([0.].into()),
                    surface_form: Partial::from_partial(PartialSurfaceVertex {
                        surface,
                        ..Default::default()
                    }),
                    ..vertex
                }
                .build(&mut services.objects)
                .insert(&mut services.objects)
            }),
            valid.global_form().clone(),
        );

        assert!(valid.validate().is_ok());
        assert!(invalid.validate().is_err());
    }
}
