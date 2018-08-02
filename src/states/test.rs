use amethyst::assets::Loader;
use amethyst::renderer::mouse::set_mouse_cursor_none;
use amethyst::renderer::{PngFormat,Sprite,SpriteRenderData};
use amethyst::{GameData, State, StateData, Trans};
use amethyst_extra::Music;
use amethyst::ecs::SystemData;
use amethyst::core::Transform;
use amethyst::prelude::*;
use amethyst::ui::UiCreator;

use utils::*;
use data::{BulletRes,BulletEmitter};

pub struct TestState;

impl<'a, 'b> State<GameData<'a, 'b>> for TestState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        data.world.add_resource(Music {
            music: vec![].into_iter().cycle(),
        });

        let _ = create_default_ortho_camera(&mut data.world);
        create_mouse_cursor(&mut data.world);
        //data.world.add_resource(ActiveCamera{entity: cam});

        set_mouse_cursor_none(&mut data.world.write_resource());

        let (mesh,material) = mesh_material_single_png(&mut data.world, "sprites/test_particle.png", (64.0,64.0));
        let bullet_res = BulletRes { mesh: mesh.clone(), material: material.clone() };


        // emitter test
        let mut tr = Transform::default();
        tr.translation = [0.1,0.1,0.0].into();
        data.world.create_entity()
            .with(tr)
            .with(BulletEmitter{bullet_material: material.clone(), bullet_mesh: mesh.clone()})
            .build();

        let mut tr = Transform::default();
        tr.translation = [0.9,0.9,0.0].into();
        data.world.create_entity()
            .with(tr)
            .with(BulletEmitter{bullet_material: material.clone(), bullet_mesh: mesh.clone()})
            .build();


        /*data.world.exec(|mut creator: UiCreator| {
            creator.create("assets/base/ui/gameplay.ron", ());
        });*/


        data.world.add_resource(bullet_res);
    }
    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(data.world);
        Trans::None
    }
}
