use crate::prelude::*;

pub struct World {
    light: Option<PointLight>,
    objects: Vec<Sphere>,
}

impl World {
    pub fn new() -> Self {
        World {
            light: None,
            objects: Vec::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_world_is_empty() {
        let world = World::new();

        assert!(world.light.is_none());
        assert!(world.objects.is_empty());
    }
}
