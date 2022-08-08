use bevy::{
    asset::HandleId,
    core_pipeline::clear_color::ClearColorConfig,
    prelude::{
        default, shape, App, AssetServer, Assets, Camera, Camera2d, Camera2dBundle, ClearColor,
        Color, Commands, Component, Handle, Image, ImageBundle, Input, KeyCode, Mesh, Query, Res,
        ResMut, Transform, Vec2, With,
    },
    reflect::TypeUuid,
    render::{
        camera::RenderTarget,
        render_resource::{
            AsBindGroup, Extent3d, ShaderRef, TextureDescriptor, TextureDimension, TextureFormat,
            TextureUsages,
        },
        view::RenderLayers,
    },
    sprite::{ColorMaterial, Material2d, Material2dPlugin, MaterialMesh2dBundle, SpriteBundle},
    DefaultPlugins,
};

#[derive(TypeUuid, AsBindGroup, Clone)]
#[uuid = "605ccb02-194c-4c4d-8a96-df5dbb7743be"]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "fragment_shader.wgsl".into()
    }
}

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "2bc7ca53-6588-4712-b3c7-de6e2ffd16f1"]
pub struct TextureMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    texture: Handle<Image>,
}

impl Material2d for TextureMaterial {
    fn fragment_shader() -> ShaderRef {
        "texture_fragment.wgsl".into()
    }
}

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "58abbb5d-e6cf-4917-bcdc-247900b10283"]
pub struct LightMaterial {
    #[texture(0)]
    #[sampler(1)]
    texture: Handle<Image>,
}

impl Material2d for LightMaterial {
    fn fragment_shader() -> ShaderRef {
        "light_shader.wgsl".into()
    }
}

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "39e8922e-03d8-4032-832e-f162354cf8f7"]
pub struct WaterMaterial {}

impl Material2d for WaterMaterial {
    fn fragment_shader() -> ShaderRef {
        "water_shader.wgsl".into()
    }
}

#[derive(Component)]
struct TextureCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Material2dPlugin::<CustomMaterial>::default())
        .add_plugin(Material2dPlugin::<TextureMaterial>::default())
        .add_plugin(Material2dPlugin::<LightMaterial>::default())
        .add_plugin(Material2dPlugin::<WaterMaterial>::default())
        .add_startup_system(setup)
        .add_system(move_camera)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    mut texture_materials: ResMut<Assets<TextureMaterial>>,
    mut light_materials: ResMut<Assets<LightMaterial>>,
    mut water_materials: ResMut<Assets<WaterMaterial>>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    // commands.spawn_bundle(MaterialMesh2dBundle {
    //     mesh: meshes.add(Mesh::from(shape::Quad {
    //         size: Vec2::new(100.0, 100.0),
    //         flip: false,
    //     })).into(),
    //     material: custom_materials.add(CustomMaterial {
    //         color: Color::GREEN,
    //     }),
    //     // transform: Transform::from_xyz(200.0, 0.0, 0.0),
    //     ..default()
    // });

    // commands.spawn_bundle(MaterialMesh2dBundle {
    //     mesh: meshes.add(Mesh::from(shape::Quad {
    //         size: Vec2::new(100.0, 100.0),
    //         flip: false,
    //     })).into(),
    //     material: texture_materials.add(TextureMaterial {
    //         color: Color::NONE,
    //         texture: asset_server.load("face.png"),
    //     }),
    //     transform: Transform::from_xyz(-200.0, 0.0, 0.0),
    //     ..default()
    // });

    // commands.spawn_bundle(MaterialMesh2dBundle {
    //     mesh: meshes.add(Mesh::from(shape::Quad {
    //         size: Vec2::new(100.0, 100.0),
    //         flip: false,
    //     })).into(),
    //     material: light_materials.add(LightMaterial {
    //         texture: asset_server.load("light.png"),
    //     }),
    //     transform: Transform::from_xyz(0.0, 0.0, 1.0),
    //     ..default()
    // });

    let size = Extent3d {
        width: 128,
        height: 128,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);
    let image_handle = images.add(image);
    let id = image_handle.id;

    commands
        .spawn_bundle(Camera2dBundle {
            camera: Camera {
                target: RenderTarget::Image(image_handle.clone()),
                depth_calculation: bevy::render::camera::DepthCalculation::ZDifference,
                priority: -1,
                ..Default::default()
            },
            transform: Transform::from_xyz(0.5, 200.0, 1.0),
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::BLUE),
            },
            ..Default::default()
        })
        .insert(RenderLayers::layer(1))
        .insert(TextureCamera);

    commands.spawn_bundle(SpriteBundle {
        texture: image_handle.clone(),
        transform: Transform::from_xyz(5.0, 0.0, 0.0),
        ..Default::default()
    });

    let mut quad = Mesh::from(shape::Quad {
        size: Vec2::new(100.0, 100.0),
        flip: false,
    });
    quad.insert_attribute(
        Mesh::ATTRIBUTE_COLOR,
        vec![
            [1.0, 0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0, 1.0],
            [0.0, 0.0, 1.0, 1.0],
            [1.0, 1.0, 1.0, 1.0],
        ],
    );
    dbg!(&quad);

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(quad).into(),
            material: water_materials.add(WaterMaterial {}),
            transform: Transform::from_xyz(0.5, 200.0, 0.0),
            ..default()
        })
        .insert(RenderLayers::layer(1));

    commands.spawn_bundle(Camera2dBundle::default()).insert(RenderLayers::all());
}

fn move_camera(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_transform: Query<&mut Transform, With<TextureCamera>>,
) {
    let mut transform = camera_transform.iter_mut().next().unwrap();
    if keyboard_input.pressed(KeyCode::Z) {
        transform.translation.y += 5.0;
    }
    if keyboard_input.pressed(KeyCode::Q) {
        transform.translation.x -= 5.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        transform.translation.y -= 5.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        transform.translation.x += 5.0;
    }
    println!(
        "x: {}, y: {}",
        transform.translation.x, transform.translation.y
    );
}
