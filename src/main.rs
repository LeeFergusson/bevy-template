use bevy::{prelude::*, window::PrimaryWindow};

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup_camera)
    .add_systems(Startup, spawn_logo)
    .run();
}

pub fn setup_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
  let window = window_query.get_single().unwrap();

  commands.spawn(Camera2dBundle {
    transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    ..Default::default()
  });
}

#[derive(Component)]
struct Logo {}

fn spawn_logo(
  mut commands: Commands,
  window_query: Query<&Window, With<PrimaryWindow>>,
  asset_server: Res<AssetServer>,
) {
  let window = window_query.get_single().unwrap();

  commands.spawn((
    SpriteBundle {
      transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
      texture: asset_server.load("sprites/bevy_logo_dark.png"),
      ..Default::default()
    },
    Logo {},
  ));
}
