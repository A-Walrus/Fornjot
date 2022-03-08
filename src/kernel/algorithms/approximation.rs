use std::collections::HashSet;

use crate::{
    kernel::{
        shape::handle::Handle,
        topology::{
            edges::{Cycle, Edge},
            faces::Face,
            vertices::Vertex,
        },
    },
    math::{Point, Scalar, Segment},
};

/// An approximation of an edge, multiple edges, or a face
#[derive(Debug, PartialEq)]
pub struct Approximation {
    /// All points that make up the approximation
    ///
    /// These could be actual vertices from the model, points that approximate
    /// an edge, or points that approximate a face.
    pub points: HashSet<Point<3>>,

    /// Segments that approximate edges
    ///
    /// Every approximation will involve edges, typically, and these are
    /// approximated by these segments.
    ///
    /// All the points of these segments will also be available in the `points`
    /// field of this struct.
    pub segments: HashSet<Segment<3>>,
}

impl Approximation {
    /// Compute an approximate for an edge
    ///
    /// `tolerance` defines how far the approximation is allowed to deviate from
    /// the actual edge.
    pub fn for_edge(edge: &Edge, tolerance: Scalar) -> Self {
        let mut points = Vec::new();
        edge.curve.approx(tolerance, &mut points);

        approximate_edge(points, edge.vertices.as_ref())
    }

    /// Compute an approximation for a cycle
    ///
    /// `tolerance` defines how far the approximation is allowed to deviate from
    /// the actual cycle.
    pub fn for_cycle(cycle: &Cycle, tolerance: Scalar) -> Self {
        let mut points = HashSet::new();
        let mut segments = HashSet::new();

        for edge in &cycle.edges {
            let approx = Self::for_edge(edge, tolerance);

            points.extend(approx.points);
            segments.extend(approx.segments);
        }

        Self { points, segments }
    }

    /// Compute an approximation for a face
    ///
    /// `tolerance` defines how far the approximation is allowed to deviate from
    /// the actual edges.
    pub fn for_face(face: &Face, tolerance: Scalar) -> Self {
        // Curved faces whose curvature is not fully defined by their edges
        // are not supported yet. For that reason, we can fully ignore `face`'s
        // `surface` field and just pass the edges to `Self::for_edges`.
        //
        // An example of a curved face that is supported, is the cylinder. Its
        // curvature is fully defined be the edges (circles) that border it. The
        // circle approximations are sufficient to triangulate the surface.
        //
        // An example of a curved face that is currently not supported, and thus
        // doesn't need to be handled here, is a sphere. A spherical face would
        // would need to provide its own approximation, as the edges that bound
        // it have nothing to do with its curvature.
        match face {
            Face::Face { surface: _, cycles } => {
                let mut points = HashSet::new();
                let mut segments = HashSet::new();

                for cycle in cycles {
                    let approx = Self::for_cycle(cycle, tolerance);

                    points.extend(approx.points);
                    segments.extend(approx.segments);
                }

                Self { points, segments }
            }
            _ => {
                // No code that still uses triangle representation calls this
                // method.
                unreachable!()
            }
        }
    }
}

fn approximate_edge(
    mut points: Vec<Point<3>>,
    vertices: Option<&[Handle<Vertex>; 2]>,
) -> Approximation {
    // Insert the exact vertices of this edge into the approximation. This means
    // we don't rely on the curve approximation to deliver accurate
    // representations of these vertices, which they might not be able to do.
    //
    // If we used inaccurate representations of those vertices here, then that
    // would lead to bugs in the approximation, as points that should refer to
    // the same vertex would be understood to refer to very close, but distinct
    // vertices.
    if let Some([a, b]) = vertices {
        points.insert(0, a.point());
        points.push(b.point());
    }

    let mut segment_points = points.clone();
    if vertices.is_none() {
        // The edge has no vertices, which means it connects to itself. We need
        // to reflect that in the approximation.

        if let Some(&point) = points.first() {
            segment_points.push(point);
        }
    }

    let mut segments = HashSet::new();
    for segment in segment_points.windows(2) {
        let p0 = segment[0];
        let p1 = segment[1];

        segments.insert(Segment::from([p0, p1]));
    }

    Approximation {
        points: points.into_iter().collect(),
        segments,
    }
}

#[cfg(test)]
mod tests {
    use map_macro::set;

    use crate::{
        kernel::{
            geometry::Surface,
            shape::Shape,
            topology::{
                edges::{Cycle, Edge},
                faces::Face,
            },
        },
        math::{Point, Scalar, Segment},
    };

    use super::{approximate_edge, Approximation};

    #[test]
    fn for_edge() {
        // Doesn't test `Approximation::for_edge` directly, but that method only
        // contains a bit of additional glue code that is not critical.

        let mut shape = Shape::new();

        let a = Point::from([1., 2., 3.]);
        let b = Point::from([2., 3., 5.]);
        let c = Point::from([3., 5., 8.]);
        let d = Point::from([5., 8., 13.]);

        let v1 = shape.vertices().add(a);
        let v2 = shape.vertices().add(d);

        let points = vec![b, c];

        // Regular edge
        assert_eq!(
            approximate_edge(points.clone(), Some(&[v1, v2])),
            Approximation {
                points: set![a, b, c, d],
                segments: set![
                    Segment::from([a, b]),
                    Segment::from([b, c]),
                    Segment::from([c, d]),
                ],
            }
        );

        // Continuous edge
        assert_eq!(
            approximate_edge(points, None),
            Approximation {
                points: set![b, c],
                segments: set![Segment::from([b, c]), Segment::from([c, b])],
            }
        );
    }

    #[test]
    fn for_cycle() {
        let tolerance = Scalar::ONE;

        let mut shape = Shape::new();

        let a = Point::from([1., 2., 3.]);
        let b = Point::from([2., 3., 5.]);
        let c = Point::from([3., 5., 8.]);

        let v1 = shape.vertices().add(a);
        let v2 = shape.vertices().add(b);
        let v3 = shape.vertices().add(c);

        let ab = shape
            .edges()
            .add(Edge::line_segment([v1.clone(), v2.clone()]));
        let bc = shape.edges().add(Edge::line_segment([v2, v3.clone()]));
        let ca = shape.edges().add(Edge::line_segment([v3, v1]));

        let cycle = Cycle {
            edges: vec![ab, bc, ca],
        };

        assert_eq!(
            Approximation::for_cycle(&cycle, tolerance),
            Approximation {
                points: set![a, b, c],
                segments: set![
                    Segment::from([a, b]),
                    Segment::from([b, c]),
                    Segment::from([c, a]),
                ],
            }
        );
    }

    #[test]
    fn for_face_closed() {
        // Test a closed face, i.e. one that is completely encircled by edges.

        let tolerance = Scalar::ONE;

        let mut shape = Shape::new();

        let a = Point::from([1., 2., 3.]);
        let b = Point::from([2., 3., 5.]);
        let c = Point::from([3., 5., 8.]);
        let d = Point::from([5., 8., 13.]);

        let v1 = shape.vertices().add(a);
        let v2 = shape.vertices().add(b);
        let v3 = shape.vertices().add(c);
        let v4 = shape.vertices().add(d);

        let ab = shape
            .edges()
            .add(Edge::line_segment([v1.clone(), v2.clone()]));
        let bc = shape.edges().add(Edge::line_segment([v2, v3.clone()]));
        let cd = shape.edges().add(Edge::line_segment([v3, v4.clone()]));
        let da = shape.edges().add(Edge::line_segment([v4, v1]));

        let abcd = shape.cycles().add(Cycle {
            edges: vec![ab, bc, cd, da],
        });

        let surface = shape.surfaces().add(Surface::x_y_plane());
        let face = Face::Face {
            surface,
            cycles: vec![abcd],
        };

        assert_eq!(
            Approximation::for_face(&face, tolerance),
            Approximation {
                points: set![a, b, c, d],
                segments: set![
                    Segment::from([a, b]),
                    Segment::from([b, c]),
                    Segment::from([c, d]),
                    Segment::from([d, a]),
                ],
            }
        );
    }
}
