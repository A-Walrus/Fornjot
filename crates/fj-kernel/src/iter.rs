//! API for iterating over the objects of a shape, or part of a shape

use std::collections::VecDeque;

use crate::{
    objects::{
        Curve, Cycle, Face, GlobalCurve, GlobalVertex, HalfEdge, Shell, Sketch,
        Solid, Surface, Vertex,
    },
    storage::Handle,
};

/// Access iterators over all objects of a shape, or part of it
///
/// Implemented for all object types. An implementation must return itself, in
/// addition to any other objects it references.
pub trait ObjectIters<'r> {
    /// Return all objects that this one references
    fn referenced_objects(&'r self) -> Vec<&'r dyn ObjectIters>;

    /// Iterate over all curves
    fn curve_iter(&'r self) -> Iter<&'r Curve> {
        let mut iter = Iter::empty();

        for object in self.referenced_objects() {
            iter = iter.with(object.curve_iter());
        }

        iter
    }

    /// Iterate over all cycles
    fn cycle_iter(&'r self) -> Iter<&'r Cycle> {
        let mut iter = Iter::empty();

        for object in self.referenced_objects() {
            iter = iter.with(object.cycle_iter());
        }

        iter
    }

    /// Iterate over all faces
    fn face_iter(&'r self) -> Iter<&'r Face> {
        let mut iter = Iter::empty();

        for object in self.referenced_objects() {
            iter = iter.with(object.face_iter());
        }

        iter
    }

    /// Iterate over all global curves
    fn global_curve_iter(&'r self) -> Iter<&'r Handle<GlobalCurve>> {
        let mut iter = Iter::empty();

        for object in self.referenced_objects() {
            iter = iter.with_handles(object.global_curve_iter());
        }

        iter
    }

    /// Iterate over all global vertices
    fn global_vertex_iter(&'r self) -> Iter<&'r GlobalVertex> {
        let mut iter = Iter::empty();

        for object in self.referenced_objects() {
            iter = iter.with(object.global_vertex_iter());
        }

        iter
    }

    /// Iterate over all half-edges
    fn half_edge_iter(&'r self) -> Iter<&'r Handle<HalfEdge>> {
        let mut iter = Iter::empty();

        for object in self.referenced_objects() {
            iter = iter.with(object.half_edge_iter());
        }

        iter
    }

    /// Iterate over all shells
    fn shell_iter(&'r self) -> Iter<&'r Shell> {
        let mut iter = Iter::empty();

        for object in self.referenced_objects() {
            iter = iter.with(object.shell_iter());
        }

        iter
    }

    /// Iterate over all sketches
    fn sketch_iter(&'r self) -> Iter<&'r Sketch> {
        let mut iter = Iter::empty();

        for object in self.referenced_objects() {
            iter = iter.with(object.sketch_iter());
        }

        iter
    }

    /// Iterate over all solids
    fn solid_iter(&'r self) -> Iter<&'r Solid> {
        let mut iter = Iter::empty();

        for object in self.referenced_objects() {
            iter = iter.with(object.solid_iter());
        }

        iter
    }

    /// Iterate over all surfaces
    fn surface_iter(&'r self) -> Iter<&'r Surface> {
        let mut iter = Iter::empty();

        for object in self.referenced_objects() {
            iter = iter.with(object.surface_iter());
        }

        iter
    }

    /// Iterator over all vertices
    fn vertex_iter(&'r self) -> Iter<&'r Handle<Vertex>> {
        let mut iter = Iter::empty();

        for object in self.referenced_objects() {
            iter = iter.with(object.vertex_iter());
        }

        iter
    }
}

impl<'r> ObjectIters<'r> for Handle<Curve> {
    fn referenced_objects(&'r self) -> Vec<&'r dyn ObjectIters> {
        vec![self.global_form() as &dyn ObjectIters]
    }

    fn curve_iter(&'r self) -> Iter<&'r Curve> {
        Iter::from_object(self)
    }
}

impl<'r> ObjectIters<'r> for Handle<Cycle> {
    fn referenced_objects(&'r self) -> Vec<&'r dyn ObjectIters> {
        let mut objects = Vec::new();

        for half_edge in self.half_edges() {
            objects.push(half_edge as &dyn ObjectIters);
        }

        objects
    }

    fn cycle_iter(&'r self) -> Iter<&'r Cycle> {
        Iter::from_object(self)
    }
}

impl<'r> ObjectIters<'r> for Handle<Face> {
    fn referenced_objects(&'r self) -> Vec<&'r dyn ObjectIters> {
        let mut objects = vec![self.surface() as &dyn ObjectIters];

        for cycle in self.all_cycles() {
            objects.push(cycle);
        }

        objects
    }

    fn face_iter(&'r self) -> Iter<&'r Face> {
        Iter::from_object(self)
    }
}

impl<'r> ObjectIters<'r> for Handle<GlobalCurve> {
    fn referenced_objects(&'r self) -> Vec<&'r dyn ObjectIters> {
        Vec::new()
    }

    fn global_curve_iter(&'r self) -> Iter<&'r Handle<GlobalCurve>> {
        Iter::from_object(self)
    }
}

impl<'r> ObjectIters<'r> for Handle<GlobalVertex> {
    fn referenced_objects(&'r self) -> Vec<&'r dyn ObjectIters> {
        Vec::new()
    }

    fn global_vertex_iter(&'r self) -> Iter<&'r GlobalVertex> {
        Iter::from_object(self)
    }
}

impl<'r> ObjectIters<'r> for Handle<HalfEdge> {
    fn referenced_objects(&'r self) -> Vec<&'r dyn ObjectIters> {
        let mut objects = vec![self.curve() as &dyn ObjectIters];

        for vertex in self.vertices().iter() {
            objects.push(vertex);
        }

        objects
    }

    fn half_edge_iter(&'r self) -> Iter<&'r Handle<HalfEdge>> {
        Iter::from_object(self)
    }
}

impl<'r> ObjectIters<'r> for Handle<Shell> {
    fn referenced_objects(&'r self) -> Vec<&'r dyn ObjectIters> {
        let mut objects = Vec::new();

        for face in self.faces() {
            objects.push(face as &dyn ObjectIters);
        }

        objects
    }

    fn shell_iter(&'r self) -> Iter<&'r Shell> {
        Iter::from_object(self)
    }
}

impl<'r> ObjectIters<'r> for Sketch {
    fn referenced_objects(&'r self) -> Vec<&'r dyn ObjectIters> {
        let mut objects = Vec::new();

        for face in self.faces() {
            objects.push(face as &dyn ObjectIters);
        }

        objects
    }

    fn sketch_iter(&'r self) -> Iter<&'r Sketch> {
        Iter::from_object(self)
    }
}

impl<'r> ObjectIters<'r> for Solid {
    fn referenced_objects(&'r self) -> Vec<&'r dyn ObjectIters> {
        let mut objects = Vec::new();

        for shell in self.shells() {
            objects.push(shell as &dyn ObjectIters);
        }

        objects
    }

    fn solid_iter(&'r self) -> Iter<&'r Solid> {
        Iter::from_object(self)
    }
}

impl<'r> ObjectIters<'r> for Handle<Surface> {
    fn referenced_objects(&'r self) -> Vec<&'r dyn ObjectIters> {
        Vec::new()
    }

    fn surface_iter(&'r self) -> Iter<&'r Surface> {
        Iter::from_object(self)
    }
}

impl<'r> ObjectIters<'r> for Handle<Vertex> {
    fn referenced_objects(&'r self) -> Vec<&'r dyn ObjectIters> {
        vec![
            self.curve() as &dyn ObjectIters,
            self.global_form() as &dyn ObjectIters,
        ]
    }

    fn vertex_iter(&'r self) -> Iter<&'r Handle<Vertex>> {
        Iter::from_object(self)
    }
}

// This implementation is useful for test code.
impl<'r, T, O> ObjectIters<'r> for T
where
    T: 'r,
    O: ObjectIters<'r> + 'r,
    &'r T: IntoIterator<Item = &'r O>,
{
    fn referenced_objects(&'r self) -> Vec<&'r dyn ObjectIters> {
        let mut objects = Vec::new();

        for object in self {
            objects.push(object as &dyn ObjectIters);
        }

        objects
    }
}

/// An iterator over objects
///
/// See [`ObjectIters`].
pub struct Iter<T>(VecDeque<T>);

impl<T> Iter<T> {
    fn empty() -> Self {
        Self(VecDeque::new())
    }

    fn from_object(object: T) -> Self {
        let mut objects = VecDeque::new();
        objects.push_back(object);
        Self(objects)
    }

    fn with(mut self, other: Self) -> Self
    where
        T: PartialEq,
    {
        for object in other {
            if !self.0.contains(&object) {
                self.0.push_back(object);
            }
        }

        self
    }
}

impl<T> Iter<&'_ Handle<T>> {
    fn with_handles(mut self, other: Self) -> Self {
        for handle in other {
            if !self.0.iter().any(|h| h.id() == handle.id()) {
                self.0.push_back(handle);
            }
        }

        self
    }
}

impl<T> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        builder::{
            CurveBuilder, CycleBuilder, FaceBuilder, HalfEdgeBuilder,
            ShellBuilder, SolidBuilder,
        },
        insert::Insert,
        objects::{GlobalCurve, GlobalVertex, Objects, SurfaceVertex, Vertex},
        partial::{
            Partial, PartialCurve, PartialCycle, PartialFace, PartialHalfEdge,
            PartialObject, PartialShell, PartialSketch, PartialSolid,
        },
        services::Services,
    };

    use super::ObjectIters as _;

    #[test]
    fn curve() {
        let mut services = Services::new();

        let surface = services.objects.surfaces.xy_plane();
        let mut object = PartialCurve {
            surface: Partial::from(surface),
            ..Default::default()
        };
        object.update_as_u_axis();
        let object = object
            .build(&mut services.objects)
            .insert(&mut services.objects);

        assert_eq!(1, object.curve_iter().count());
        assert_eq!(0, object.cycle_iter().count());
        assert_eq!(0, object.face_iter().count());
        assert_eq!(1, object.global_curve_iter().count());
        assert_eq!(0, object.global_vertex_iter().count());
        assert_eq!(0, object.half_edge_iter().count());
        assert_eq!(0, object.shell_iter().count());
        assert_eq!(0, object.sketch_iter().count());
        assert_eq!(0, object.solid_iter().count());
        assert_eq!(0, object.surface_iter().count());
        assert_eq!(0, object.vertex_iter().count());
    }

    #[test]
    fn cycle() {
        let mut services = Services::new();

        let surface = services.objects.surfaces.xy_plane();
        let object = {
            let mut cycle = PartialCycle::default();
            cycle.from_poly_chain(surface, [[0., 0.], [1., 0.], [0., 1.]]);
            cycle
                .build(&mut services.objects)
                .insert(&mut services.objects)
        };

        assert_eq!(3, object.curve_iter().count());
        assert_eq!(1, object.cycle_iter().count());
        assert_eq!(0, object.face_iter().count());
        assert_eq!(3, object.global_curve_iter().count());
        assert_eq!(3, object.global_vertex_iter().count());
        assert_eq!(3, object.half_edge_iter().count());
        assert_eq!(0, object.shell_iter().count());
        assert_eq!(0, object.sketch_iter().count());
        assert_eq!(0, object.solid_iter().count());
        assert_eq!(0, object.surface_iter().count());
        assert_eq!(6, object.vertex_iter().count());
    }

    #[test]
    fn face() {
        let mut services = Services::new();

        let surface = services.objects.surfaces.xy_plane();
        let mut object = PartialFace::default();
        object.update_exterior_as_polygon(
            surface,
            [[0., 0.], [1., 0.], [0., 1.]],
        );
        let object = object
            .build(&mut services.objects)
            .insert(&mut services.objects);

        assert_eq!(3, object.curve_iter().count());
        assert_eq!(1, object.cycle_iter().count());
        assert_eq!(1, object.face_iter().count());
        assert_eq!(3, object.global_curve_iter().count());
        assert_eq!(3, object.global_vertex_iter().count());
        assert_eq!(3, object.half_edge_iter().count());
        assert_eq!(0, object.shell_iter().count());
        assert_eq!(0, object.sketch_iter().count());
        assert_eq!(0, object.solid_iter().count());
        assert_eq!(1, object.surface_iter().count());
        assert_eq!(6, object.vertex_iter().count());
    }

    #[test]
    fn global_curve() {
        let mut services = Services::new();

        let object = GlobalCurve.insert(&mut services.objects);

        assert_eq!(0, object.curve_iter().count());
        assert_eq!(0, object.cycle_iter().count());
        assert_eq!(0, object.face_iter().count());
        assert_eq!(1, object.global_curve_iter().count());
        assert_eq!(0, object.global_vertex_iter().count());
        assert_eq!(0, object.half_edge_iter().count());
        assert_eq!(0, object.shell_iter().count());
        assert_eq!(0, object.sketch_iter().count());
        assert_eq!(0, object.solid_iter().count());
        assert_eq!(0, object.surface_iter().count());
        assert_eq!(0, object.vertex_iter().count());
    }

    #[test]
    fn global_vertex() {
        let mut services = Services::new();

        let object =
            GlobalVertex::new([0., 0., 0.]).insert(&mut services.objects);

        assert_eq!(0, object.curve_iter().count());
        assert_eq!(0, object.cycle_iter().count());
        assert_eq!(0, object.face_iter().count());
        assert_eq!(0, object.global_curve_iter().count());
        assert_eq!(1, object.global_vertex_iter().count());
        assert_eq!(0, object.half_edge_iter().count());
        assert_eq!(0, object.shell_iter().count());
        assert_eq!(0, object.sketch_iter().count());
        assert_eq!(0, object.solid_iter().count());
        assert_eq!(0, object.surface_iter().count());
        assert_eq!(0, object.vertex_iter().count());
    }

    #[test]
    fn half_edge() {
        let mut services = Services::new();

        let object = {
            let mut half_edge = PartialHalfEdge::default();
            half_edge.update_as_line_segment_from_points(
                services.objects.surfaces.xy_plane(),
                [[0., 0.], [1., 0.]],
            );

            half_edge
                .build(&mut services.objects)
                .insert(&mut services.objects)
        };

        assert_eq!(1, object.curve_iter().count());
        assert_eq!(0, object.cycle_iter().count());
        assert_eq!(0, object.face_iter().count());
        assert_eq!(1, object.global_curve_iter().count());
        assert_eq!(2, object.global_vertex_iter().count());
        assert_eq!(1, object.half_edge_iter().count());
        assert_eq!(0, object.shell_iter().count());
        assert_eq!(0, object.sketch_iter().count());
        assert_eq!(0, object.solid_iter().count());
        assert_eq!(0, object.surface_iter().count());
        assert_eq!(2, object.vertex_iter().count());
    }

    #[test]
    fn shell() {
        let mut services = Services::new();

        let object = PartialShell::create_cube_from_edge_length(
            1.,
            &mut services.objects,
        )
        .build(&mut services.objects)
        .insert(&mut services.objects);

        assert_eq!(24, object.curve_iter().count());
        assert_eq!(6, object.cycle_iter().count());
        assert_eq!(6, object.face_iter().count());
        assert_eq!(12, object.global_curve_iter().count());
        assert_eq!(8, object.global_vertex_iter().count());
        assert_eq!(24, object.half_edge_iter().count());
        assert_eq!(1, object.shell_iter().count());
        assert_eq!(0, object.sketch_iter().count());
        assert_eq!(0, object.solid_iter().count());
        assert_eq!(6, object.surface_iter().count());
        assert_eq!(48, object.vertex_iter().count());
    }

    #[test]
    fn sketch() {
        let mut services = Services::new();

        let surface = services.objects.surfaces.xy_plane();
        let mut face = PartialFace::default();
        face.update_exterior_as_polygon(
            surface,
            [[0., 0.], [1., 0.], [0., 1.]],
        );
        let object = PartialSketch {
            faces: vec![Partial::from_partial(face)],
        }
        .build(&mut services.objects);

        assert_eq!(3, object.curve_iter().count());
        assert_eq!(1, object.cycle_iter().count());
        assert_eq!(1, object.face_iter().count());
        assert_eq!(3, object.global_curve_iter().count());
        assert_eq!(3, object.global_vertex_iter().count());
        assert_eq!(3, object.half_edge_iter().count());
        assert_eq!(0, object.shell_iter().count());
        assert_eq!(1, object.sketch_iter().count());
        assert_eq!(0, object.solid_iter().count());
        assert_eq!(1, object.surface_iter().count());
        assert_eq!(6, object.vertex_iter().count());
    }

    #[test]
    fn solid() {
        let mut services = Services::new();

        let object = {
            let mut solid = PartialSolid::default();
            solid.with_cube_from_edge_length(1., &mut services.objects);
            solid.build(&mut services.objects)
        };

        assert_eq!(24, object.curve_iter().count());
        assert_eq!(6, object.cycle_iter().count());
        assert_eq!(6, object.face_iter().count());
        assert_eq!(12, object.global_curve_iter().count());
        assert_eq!(8, object.global_vertex_iter().count());
        assert_eq!(24, object.half_edge_iter().count());
        assert_eq!(1, object.shell_iter().count());
        assert_eq!(0, object.sketch_iter().count());
        assert_eq!(1, object.solid_iter().count());
        assert_eq!(6, object.surface_iter().count());
        assert_eq!(48, object.vertex_iter().count());
    }

    #[test]
    fn surface() {
        let objects = Objects::new();

        let object = objects.surfaces.xy_plane();

        assert_eq!(0, object.curve_iter().count());
        assert_eq!(0, object.cycle_iter().count());
        assert_eq!(0, object.face_iter().count());
        assert_eq!(0, object.global_curve_iter().count());
        assert_eq!(0, object.global_vertex_iter().count());
        assert_eq!(0, object.half_edge_iter().count());
        assert_eq!(0, object.shell_iter().count());
        assert_eq!(0, object.sketch_iter().count());
        assert_eq!(0, object.solid_iter().count());
        assert_eq!(1, object.surface_iter().count());
        assert_eq!(0, object.vertex_iter().count());
    }

    #[test]
    fn vertex() {
        let mut services = Services::new();

        let surface = services.objects.surfaces.xy_plane();
        let mut curve = PartialCurve {
            surface: Partial::from(surface.clone()),
            ..Default::default()
        };
        curve.update_as_u_axis();
        let curve = curve
            .build(&mut services.objects)
            .insert(&mut services.objects);
        let global_vertex =
            GlobalVertex::new([0., 0., 0.]).insert(&mut services.objects);
        let surface_vertex =
            SurfaceVertex::new([0., 0.], surface, global_vertex)
                .insert(&mut services.objects);
        let object = Vertex::new([0.], curve, surface_vertex)
            .insert(&mut services.objects);

        assert_eq!(1, object.curve_iter().count());
        assert_eq!(0, object.cycle_iter().count());
        assert_eq!(0, object.face_iter().count());
        assert_eq!(1, object.global_curve_iter().count());
        assert_eq!(1, object.global_vertex_iter().count());
        assert_eq!(0, object.half_edge_iter().count());
        assert_eq!(0, object.shell_iter().count());
        assert_eq!(0, object.sketch_iter().count());
        assert_eq!(0, object.solid_iter().count());
        assert_eq!(0, object.surface_iter().count());
        assert_eq!(1, object.vertex_iter().count());
    }
}
