use amethyst::assets::Handle;
use amethyst::ecs::{Component, DenseVecStorage, VecStorage};
use amethyst::renderer::{Mesh, Texture};

pub struct BulletEmitter {
    pub bullet_texture: Handle<Texture>,
    pub bullet_mesh: Handle<Mesh>,
}

impl Component for BulletEmitter {
    type Storage = VecStorage<Self>;
}
