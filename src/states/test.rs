use amethyst::assets::Loader;
use amethyst::renderer::mouse::set_mouse_cursor_none;
use amethyst::renderer::PngFormat;
use amethyst::{GameData, State, StateData, Trans};
use amethyst_extra::Music;

use utils::{create_default_ortho_camera, create_mouse_cursor};

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

        let sprite = Sprite {
            left: 0.,
            right: 64.,
            top: 0.,
            bottom: 64.,
        };

        let bullet_res = BulletRes { texture, sprite };
        data.world.add_resource(bullet_res);
    }
    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(data.world);
        Trans::None
    }
}
