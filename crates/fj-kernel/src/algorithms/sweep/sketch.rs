use fj_math::Vector;

use crate::{
    insert::Insert,
    objects::{Objects, Sketch, Solid},
    services::Service,
    storage::Handle,
};

use super::{Sweep, SweepCache};

impl Sweep for Handle<Sketch> {
    type Swept = Handle<Solid>;

    fn sweep_with_cache(
        self,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        objects: &mut Service<Objects>,
    ) -> Self::Swept {
        let path = path.into();

        let mut shells = Vec::new();
        for face in self.faces().clone() {
            let shell = face.sweep_with_cache(path, cache, objects);
            shells.push(shell);
        }

        Solid::new(shells).insert(objects)
    }
}
