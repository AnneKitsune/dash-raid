use amethyst::assets::Loader;
use amethyst::renderer::mouse::set_mouse_cursor_none;
use amethyst::renderer::{PngFormat,Sprite,SpriteRenderData};
use amethyst::{GameData, State, StateData, Trans};
use amethyst_extra::Music;
use amethyst::ecs::SystemData;

use utils::*;
use data::BulletRes;

pub struct TestState;

impl<'a, 'b> State<GameData<'a, 'b>> for TestState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        data.world.add_resource(Music {
            music: vec![].into_iter().cycle(),
        });

        let _ = create_default_ortho_camera(&mut data.world);
        create_mouse_cursor(&mut data.world);
        //data.world.add_resource(ActiveCamera{entity: cam});

        //set_mouse_cursor_none(&mut data.world.write_resource());

        let (mesh,material) = mesh_material_single_png(&mut data.world, "sprites/test_particle.png", (64.0,64.0));
        let bullet_res = BulletRes { mesh, material };
        data.world.add_resource(bullet_res);
    }
    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(data.world);
        Trans::None
    }
}
