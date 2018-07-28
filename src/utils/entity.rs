use amethyst::core::cgmath::{Matrix4, Vector3};
use amethyst::core::{GlobalTransform, Transform};
use amethyst::ecs::{Entity, World,SystemData};
use amethyst::prelude::Builder;
use amethyst::assets::Handle;
use amethyst::renderer::{Camera, PngFormat, Projection,Sprite,WithSpriteRender,Texture,Mesh,SpriteRenderData,Material};
use amethyst_extra::{AssetLoader, AssetLoaderInternal, FollowMouse};

pub fn create_mouse_cursor(world: &mut World) -> Entity {
    let mut tr = Transform::default();
    tr.scale = [0.001, 0.001, 0.001].into();
    
    let (mesh,mat) = mesh_material_single_png(world, "sprites/test_particle.png", (64.0,64.0));
    world
        .create_entity()
        .with(tr)
        .with(GlobalTransform::default())
        .with(FollowMouse)
        .with(mesh)
        .with(mat)
        .build()
}

pub fn mesh_material_single_png(world: &mut World, image_path: &str, size: (f32,f32)) -> (Handle<Mesh>, Material) {
    if world.res.try_fetch::<AssetLoaderInternal<Texture>>().is_none(){
        world.add_resource(AssetLoaderInternal::<Texture>::new());
    }
    let texture = {
        let asset_loader = world.read_resource::<AssetLoader>();
        //&mut {world.res.entry().or_insert_with(|| AssetLoaderInternal::new())},
        asset_loader
            .load(
                image_path,
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
        right: size.0,
        top: 0.,
        bottom: size.1,
    };
    SpriteRenderData::setup(&mut world.res);
    SpriteRenderData::fetch(&mut world.res).build_mesh_and_material(&sprite,texture,size)
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
