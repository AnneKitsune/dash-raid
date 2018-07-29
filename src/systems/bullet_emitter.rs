use amethyst::core::timing::Time;
use amethyst::core::{GlobalTransform, Transform};
use amethyst::ecs::{Entities, Read, ReadExpect, System, WriteStorage,ReadStorage,Join};
use amethyst_extra::DestroyInTime;
use amethyst::renderer::{MeshHandle,Material,Transparent};
use rand::{thread_rng, Rng};

use data::{Bullet, BulletRes, BulletEmitter};

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
        WriteStorage<'a, MeshHandle>,
        WriteStorage<'a, Material>,
        ReadStorage<'a, BulletEmitter>,
        WriteStorage<'a,Transparent>,
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
            mut meshes,
            mut materials,
            bullet_emitters,
            mut transparents,
        ): Self::SystemData,
    ) {
        if time.absolute_time_seconds() > self.last_spawn + 0.2 {
            self.last_spawn = time.absolute_time_seconds();

            let mut holder = vec![];
            for (transform,bullet_emitter) in (&transforms, &bullet_emitters).join() {
                holder.push((transform.translation, bullet_emitter.bullet_material.clone(),bullet_emitter.bullet_mesh.clone()))
            }
            while let Some(e) = holder.pop(){
                let mut rng = thread_rng();
                let vel = [rng.gen_range(-0.05, 0.05), rng.gen_range(-0.05, 0.05), 0.0];

                let mut tr = Transform::default();
                tr.translation = e.0;
                tr.scale = [0.0001, 0.0001, 1.0].into();
                let new_bullet = entities
                    .build_entity()
                    .with(tr, &mut transforms)
                    .with(GlobalTransform::default(), &mut global_transforms)
                    .with(
                        Bullet {
                            velocity: vel.into(),
                        },
                        &mut bullets,
                    )
                    .with(Transparent, &mut transparents)
                    .with(DestroyInTime { timer: 5.0 }, &mut destroy_in_times)
                    .with(e.2, &mut meshes)
                    .with(e.1, &mut materials)
                    .build();
            }
        }
    }
}
