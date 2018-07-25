use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};

use data::Bullet;

#[derive(Default)]
pub struct BulletMoverSystem;

impl<'a> System<'a> for BulletMoverSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Bullet>,
        Read<'a, Time>,
    );
    fn run(&mut self, (mut transforms, bullets, time): Self::SystemData) {
        for (mut transform, bullet) in (&mut transforms, &bullets).join() {
            transform.translation += bullet.velocity * time.delta_seconds();
        }
    }
}
