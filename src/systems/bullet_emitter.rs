use amethyst::core::timing::Time;
use amethyst::core::{GlobalTransform, Transform};
use amethyst::ecs::{Entities, Read, ReadExpect, System, WriteStorage};
use amethyst_extra::DestroyInTime;
use rand::{thread_rng, Rng};

use data::Bullet;

#[derive(Default)]
pub struct BulletEmitterSystem {
    last_spawn: f64,
}

impl<'a> System<'a> for BulletEmitterSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, Time>,
        ReadExpect<'a, BulletRes>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, GlobalTransform>,
        WriteStorage<'a, Bullet>,
        WriteStorage<'a, DestroyInTime>,
        SpriteRenderData<'a>,
    );
    fn run(
        &mut self,
        (
            entities,
            time,
            bullet_res,
            mut transforms,
            mut global_transforms,
            mut bullets,
            mut destroy_in_times,
            mut sprite_render_data,
        ): Self::SystemData,
    ) {
        if time.absolute_time_seconds() > self.last_spawn + 0.05 {
            // TODO: Make it a resource
            let mut rng = thread_rng();

            self.last_spawn = time.absolute_time_seconds();

            let mut tr = Transform::default();
            tr.translation = [0.5, 0.5, 0.0].into();
            tr.scale = [0.001, 0.001, 0.001].into();
            let new_bullet = entities
                .build_entity()
                .with(tr, &mut transforms)
                .with(GlobalTransform::default(), &mut global_transforms)
                .with(
                    Bullet {
                        velocity: [rng.gen_range(-0.01, 0.01), rng.gen_range(-0.01, 0.01), 0.0]
                            .into(),
                    },
                    &mut bullets,
                )
                .with(DestroyInTime { timer: 5.0 }, &mut destroy_in_times)
                .build();
            sprite_render_data
                .add(
                    new_bullet,
                    &bullet_res.sprite,
                    bullet_res.texture.clone(),
                    (64.0, 64.0),
                )
                .expect("Failed to build test sprite");
        }
    }
}
