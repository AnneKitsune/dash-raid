use amethyst::ecs::{Component, VecStorage};
use amethyst::assets::Handle;
use amethyst::renderer::{Mesh,Material};

#[derive(Clone)]
/// The BulletRes `Resource`.
pub struct BulletRes {
    pub mesh: Handle<Mesh>,
    pub material: Material,
}

impl BulletRes {
    
    /// Creates a new BulletRes.
    pub fn new(mesh: Handle<Mesh>, material: Material) -> Self {
        BulletRes {
            mesh,
            material
        }
    }
}

impl Component for BulletRes{
    type Storage = VecStorage<Self>;
}


