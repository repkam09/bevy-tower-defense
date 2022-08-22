use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

pub const DEFAULT_HEIGHT: f32 = 720.0;
pub const DEFAULT_WIDTH: f32 = 1280.0;

fn main() {
    // Uses the builder pattern. Defaults create a window, input handling, etc
    // This is likely the only things you need in main for most projects
    App::new()
        .insert_resource(ClearColor(Color::BEIGE))
        .insert_resource(WindowDescriptor {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            title: "Bevy Tower Defense".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(spawn_camera)
        .add_startup_system(asset_loading)
        .add_system(tower_shooting)
        .add_system(bullet_despawn)
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .register_type::<Tower>()
        .run();
}

// Spawns a camera, should run once at startup via add_startup_system
fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .insert(Name::new("Ground"));

    // Sawn a cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert(Tower {
            shooting_timer: Timer::from_seconds(1.0, true),
        })
        .insert(Name::new("Tower"));

    // Spawn a light
    commands
        .spawn_bundle(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..Default::default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..Default::default()
        })
        .insert(Name::new("Light"));
}

#[derive(Default, Reflect, Component)]
#[reflect(Component)]
pub struct Tower {
    shooting_timer: Timer,
}

#[derive(Default, Reflect, Component)]
#[reflect(Component)]
pub struct Lifetime {
    timer: Timer,
}

fn tower_shooting(
    mut commands: Commands,
    bullet_assets: Res<GameAssets>,
    mut towers: Query<&mut Tower>,
    time: Res<Time>,
) {
    for mut tower in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let spawn_transform =
                Transform::from_xyz(0.0, 0.7, 0.6).with_rotation(Quat::from_rotation_y(-PI / 2.0));

            commands
                .spawn_bundle(SceneBundle {
                    scene: bullet_assets.bullet_scene.clone(),
                    transform: spawn_transform,
                    ..Default::default()
                })
                .insert(Lifetime {
                    timer: Timer::from_seconds(0.5, false),
                })
                .insert(Name::new("Bullet"));
        }
    }
}

fn bullet_despawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut bullets {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bullet_scene: assets.load("Bullet.glb#Scene0"),
    });
}

pub struct GameAssets {
    bullet_scene: Handle<Scene>,
}
