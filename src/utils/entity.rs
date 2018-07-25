use amethyst::core::cgmath::{Matrix4, Vector3};
use amethyst::core::{GlobalTransform, Transform};
use amethyst::ecs::{Entity, World};
use amethyst::prelude::Builder;
use amethyst::renderer::{Camera, PngFormat, Projection};
use amethyst_extra::{AssetLoader, FollowMouse};

pub fn create_mouse_cursor(world: &mut World) -> Entity {
    let mut tr = Transform::default();
    tr.scale = [0.001, 0.001, 0.001].into();

    let cursor_texture = {
        let asset_loader = world.read_resource::<AssetLoader>();
        asset_loader
            .load(
                "sprites/test_particle.png",
                PngFormat,
                Default::default(),
                &mut world.write_resource(),
                &mut world.write_resource(),
                &world.read_resource(),
            )
            .unwrap()
    };

    let sprite = Sprite {
        left: 0.,
        right: 64.,
        top: 0.,
        bottom: 64.,
    };

    world
        .create_entity()
        .with(tr)
        .with(GlobalTransform::default())
        .with(FollowMouse)
        .with_sprite(&sprite, cursor_texture, (64.0, 64.0))
        .unwrap()
        .build()
}

pub fn create_default_ortho_camera(world: &mut World) -> Entity {
    world
        .create_entity()
        .with(GlobalTransform(
            Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)).into(),
        ))
        .with(Camera::from(Projection::orthographic(0., 1., 1., 0.)))
        .build()
}
