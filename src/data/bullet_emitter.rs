use amethyst::assets::Handle;
use amethyst::ecs::{Component, DenseVecStorage, VecStorage};
use amethyst::renderer::{Mesh, Material};

#[derive(Clone)]
pub struct BulletEmitter {
    pub bullet_material: Material,
    pub bullet_mesh: Handle<Mesh>,
}

impl Component for BulletEmitter {
    type Storage = VecStorage<Self>;
}
