extern crate amethyst;
extern crate amethyst_extra;
#[macro_use]
extern crate log;
extern crate rand;

use amethyst_extra::*;
use amethyst::prelude::*;
use amethyst::audio::*;
use amethyst::animation::*;
use amethyst::ui::*;
use amethyst::input::*;
use amethyst::ecs::*;
use amethyst::core::*;
use amethyst::assets::*;
use amethyst::renderer::*;
use amethyst::Result;
use amethyst::core::cgmath::Vector3;

use rand::{thread_rng, Rng};


struct TestState;

impl<'a,'b> State<GameData<'a,'b>> for TestState{
    fn on_start(&mut self, data: StateData<GameData>) {
        data.world.add_resource(
            Music{
                music: vec![].into_iter().cycle(),
            }
        );

        let texture = {
            let loader = data.world.read_resource::<Loader>();
            loader.load(
                "assets/base/sprites/test_particle.png",
                PngFormat,
                Default::default(),
                (),
                &data.world.read_resource(),
            )
        };

        let sprite = Sprite{
            left: 0.,
            right: 64.,
            top: 0.,
            bottom: 64.,
        };

        let bullet_res = BulletRes{
            texture,
            sprite,
        };
        data.world.add_resource(bullet_res);

    }
    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(data.world);
        Trans::None
    }

}

pub struct BulletRes{
    texture: Handle<Texture>,
    sprite: Sprite,
}
impl Component for BulletRes{
    type Storage = VecStorage<Self>;
}

pub struct Bullet{
    velocity: Vector3<f32>,
}
impl Component for Bullet{
    type Storage = DenseVecStorage<Self>;
}

pub struct BulletMoverSystem;

impl<'a> System<'a> for BulletMoverSystem{
    type SystemData = (WriteStorage<'a,Transform>,ReadStorage<'a,Bullet>);
    fn run(&mut self,(mut transforms, bullets): Self::SystemData) {
        for (mut transform, bullet) in (&mut transforms, &bullets).join(){
            transform.translation += bullet.velocity;
        }
    }
}

#[derive(Default)]
pub struct BulletEmitterSystem{
    last_spawn: f64,
}


impl<'a> System<'a> for BulletEmitterSystem{
    type SystemData = (Entities<'a>,Read<'a, Time>,ReadExpect<'a, BulletRes>,
                       WriteStorage<'a, Transform>,WriteStorage<'a, GlobalTransform>,
                       WriteStorage<'a, Bullet>, WriteStorage<'a, DestroyInTime>, SpriteRenderData<'a>);
    fn run(&mut self,(entities,time,bullet_res,mut transforms,mut global_transforms,mut bullets, mut destroy_in_times, mut sprite_render_data): Self::SystemData) {
        if time.absolute_time_seconds() > self.last_spawn + 0.05 {

            // TODO: Make it a resource
            let mut rng = thread_rng();

            self.last_spawn = time.absolute_time_seconds();

            let mut tr = Transform::default();
            tr.scale = [0.001, 0.001, 0.001].into();
            let new_bullet = entities.build_entity()
                .with(tr, &mut transforms)
                .with(GlobalTransform::default(), &mut global_transforms)
                .with(Bullet { velocity: [rng.gen_range(-0.01,0.01), rng.gen_range(-0.01,0.01), 0.0].into() }, &mut bullets)
                .with(DestroyInTime { timer: 5.0 }, &mut destroy_in_times)
                .build();
            sprite_render_data.add(new_bullet, &bullet_res.sprite, bullet_res.texture.clone(), (64.0, 64.0)).expect("Failed to build test sprite");
        }
    }
}

pub fn create_default_ortho_camera(world: &mut World) {
    world.create_entity()
        .with(Transform::default())
        .with(Camera::from(Projection::orthographic(0., 1., 1., 0.)))
        .build();
}

fn main() -> Result<()>{
    //let mut game_data_builder = amethyst_gamedata_base_2d(env!("CARGO_MANIFEST_DIR")).unwrap();
    amethyst::start_logger(Default::default());

    let display_config_path = format!(
        "{}/assets/base/config/display.ron",
        env!("CARGO_MANIFEST_DIR")
    );

    let key_bindings_path = format!(
        "{}/assets/base/config/input.ron",
        env!("CARGO_MANIFEST_DIR")
    );

    let game_data_builder = GameDataBuilder::default()
        .with(BulletMoverSystem,"bullet_mover",&[])
        .with(BulletEmitterSystem::default(),"bullet_emitter",&[])
        .with_bundle(TransformBundle::new().with_dep(&["bullet_mover","bullet_emitter"]))?
        .with_bundle(
            InputBundle::<String, String>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with_bundle(UiBundle::<String, String>::new())?
        .with_bundle(
            AnimationBundle::<u32, Material>::new(
                "animation_control_system",
                "sampler_interpolation_system",
            )
        )?
        .with_bundle(AudioBundle::new(|music: &mut Music| music.music.next()))?
        .with(TimedDestroySystem,"timed_destroy", &[])
        .with(NormalOrthoCameraSystem::default(), "aspect_ratio",&[])
        .with_basic_renderer(display_config_path, DrawFlat::<PosTex>::new(), false)?;
    let resources_directory = format!("{}", env!("CARGO_MANIFEST_DIR"));
    Application::build(resources_directory, TestState)?.build(game_data_builder)?.run();
    Ok(())
}