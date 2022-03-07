use crate::{
    debug::DebugInfo,
    kernel::{
        geometry::Surface,
        shape::Shape,
        topology::{
            edges::{Cycle, Edge},
            faces::{Face, Faces},
        },
    },
    math::{Aabb, Point, Scalar},
};

use super::ToShape;

impl ToShape for fj::Sketch {
    fn to_shape(&self, _: Scalar, _: &mut DebugInfo) -> Shape {
        let mut shape = Shape::new();
        let mut vertices = Vec::new();

        for [x, y] in self.to_points() {
            let vertex = shape.vertices().add(Point::from([x, y, 0.]));
            vertices.push(vertex);
        }

        {
            if !vertices.is_empty() {
                // Add the first vertex at the end again, to close the loop.
                //
                // This can't panic. We just checked that `vertices` is not
                // empty.
                vertices.push(vertices[0].clone());
            }

            let mut edges = Vec::new();
            for window in vertices.windows(2) {
                // Can't panic, we passed `2` to `windows`.
                //
                // Can be cleaned up, once `array_windows` is stable.
                let a = window[0].clone();
                let b = window[1].clone();

                let edge = shape.edges().add(Edge::line_segment([a, b]));
                edges.push(edge);
            }

            shape.cycles().create(Cycle { edges });
        };

        let face = Face::Face {
            cycles: shape
                .cycles()
                .all()
                .map(|handle| (*handle).clone())
                .collect(),
            surface: Surface::x_y_plane(),
        };
        shape.faces = Faces(vec![face]);

        shape
    }

    fn bounding_volume(&self) -> Aabb<3> {
        Aabb::<3>::from_points(
            self.to_points()
                .into_iter()
                .map(Point::from)
                .map(Point::to_xyz),
        )
    }
}
