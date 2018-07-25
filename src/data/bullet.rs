use amethyst::assets::Handle;
use amethyst::core::cgmath::Vector3;
use amethyst::ecs::{Component, DenseVecStorage, VecStorage};
use amethyst::renderer::{Mesh, Texture};

pub struct Bullet {
    pub velocity: Vector3<f32>,
}

impl Component for Bullet {
    type Storage = DenseVecStorage<Self>;
}
