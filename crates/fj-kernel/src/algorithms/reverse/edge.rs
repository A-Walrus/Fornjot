use crate::{
    insert::Insert,
    objects::{HalfEdge, Objects},
    services::Service,
    storage::Handle,
};

use super::Reverse;

impl Reverse for Handle<HalfEdge> {
    fn reverse(self, objects: &mut Service<Objects>) -> Self {
        let vertices = {
            let [a, b] = self.vertices().clone();
            [b, a]
        };

        HalfEdge::new(
            self.curve().clone(),
            vertices,
            self.global_form().clone(),
        )
        .insert(objects)
    }
}
