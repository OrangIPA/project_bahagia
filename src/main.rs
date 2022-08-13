use bevy::prelude::*;

#[derive(Component)]
struct MainCamera;

const MOV_SPEED: f32 = 0.5;
const HORIZONTAL_SPEED: f32 = 0.3;
const TITLE: &str = ":D";

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: TITLE.to_owned(),
            ..Default::default()
        })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1. / 5.0f32,
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(camera_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_scene(asset_server.load("desbah.gltf#Scene0"));
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.7, 0.7, 1.0).looking_at(Vec3::new(0., 0.3, 0.), Vec3::Y),
        ..default()
    })
    .insert(MainCamera);
    const HALF_SIZE: f32 = 1.;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -100. * HALF_SIZE,
                far: 100. * HALF_SIZE,
                ..Default::default()
            },
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0., 40., 0.),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn camera_movement(
    mut query: Query<&mut Transform, With<MainCamera>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut cam = query.single_mut();

    if cam.rotation != Quat::IDENTITY {
        println!("{:?}", cam.rotation.to_euler(EulerRot::XYZ));
    }

    cam.rotation = Quat::IDENTITY;

    if input.pressed(KeyCode::D) {
        cam.translation.x += MOV_SPEED * time.delta_seconds();
    }

    if input.pressed(KeyCode::A) {
        cam.translation.x += -MOV_SPEED * time.delta_seconds();
    }
    
    if input.pressed(KeyCode::W) {
        cam.translation.z += -MOV_SPEED * time.delta_seconds();
    }

    if input.pressed(KeyCode::S) {
        cam.translation.z += MOV_SPEED * time.delta_seconds();
    }
    
    if input.pressed(KeyCode::Space) {
        cam.translation.y += HORIZONTAL_SPEED * time.delta_seconds();
    }

    if input.pressed(KeyCode::LShift) {
        cam.translation.y += -HORIZONTAL_SPEED * time.delta_seconds();
    }
}