use std::f64::consts::PI;

use nalgebra::vector;
use parry3d_f64::math::Isometry;

use crate::{
    debug::DebugInfo,
    kernel::{
        approximation::Approximation,
        topology::{
            edges::Edges,
            faces::{Face, Faces},
            vertices::Vertices,
        },
        Shape,
    },
    math::{Aabb, Scalar, Transform},
};

impl Shape for fj::Sweep {
    fn bounding_volume(&self) -> Aabb<3> {
        let mut aabb = self.shape.bounding_volume();
        aabb.max.z = self.length.into();
        aabb
    }

    fn faces(&self, tolerance: Scalar, debug_info: &mut DebugInfo) -> Faces {
        let rotation = Isometry::rotation(vector![PI, 0., 0.]).into();
        let translation = Isometry::translation(0.0, 0.0, self.length).into();

        let original_faces = self.shape.faces(tolerance, debug_info);

        let bottom_faces = original_faces.clone().transform(&rotation);

        let top_faces = original_faces.transform(&translation);

        let mut side_faces = Vec::new();
        for cycle in self.shape.edges().cycles {
            let approx = Approximation::for_cycle(&cycle, tolerance);

            // This will only work correctly, if the cycle consists of one edge.
            // If there are more, this will create some kind of weird face
            // chimera, a single face to represent all the side faces.

            let mut quads = Vec::new();
            for segment in approx.segments {
                let [v0, v1] = segment.points();
                let [v3, v2] = {
                    let segment = Transform::translation(0., 0., self.length)
                        .transform_segment(&segment);
                    segment.points()
                };

                quads.push([v0, v1, v2, v3]);
            }

            let mut side_face = Vec::new();
            for [v0, v1, v2, v3] in quads {
                side_face.push([v0, v1, v2].into());
                side_face.push([v0, v2, v3].into());
            }

            side_faces.push(Face::Triangles(side_face));
        }

        let mut faces = Vec::new();
        faces.extend(bottom_faces.0);
        faces.extend(top_faces.0);
        faces.extend(side_faces);

        Faces(faces)
    }

    fn edges(&self) -> Edges {
        todo!()
    }

    fn vertices(&self) -> Vertices {
        todo!()
    }
}
