use bevy::{gltf::Gltf, prelude::*};
use bevy_flycam::PlayerPlugin;

struct Model(Handle<Gltf>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(load)
        .add_system(spawn)
        .run()
}

fn load(mut commands: Commands, ass: Res<AssetServer>) {
    commands.insert_resource(Model(ass.load("untitled.gltf")));
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..PerspectiveCameraBundle::default()
    });
}

#[derive(Component)]
struct SpawnedModel;

fn spawn(
    mut commands: Commands,
    model: Res<Model>,
    gltfs: Res<Assets<Gltf>>,
    spawned_model: Query<(), With<SpawnedModel>>,
) {
    if spawned_model.get_single().is_ok() {
    } else if let Some(gltf) = gltfs.get(&model.0) {
        commands.spawn_scene(gltf.scenes[0].clone());
    }
}
