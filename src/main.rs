use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(move_player)
        .run();
}

#[derive(Component)]
pub struct Player;

pub fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>
) {
   commands.spawn_bundle(Camera3dBundle {
    transform: Transform::from_xyz(2.0, 10.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ..default()
   });

   commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Cube {size: 1.0})),
    material: materials.add(Color::rgb(1.0,0.0,0.0).into()),
    transform: Transform::from_xyz(0.0,2.0,0.0),
    ..default()
   })
  .insert(Player)
  .insert(Name::new("Cube"));

  commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Plane {size: 40.0})),
    material: materials.add(Color::rgb(1.0,0.0,1.0).into()),
    ..default()
   })
  .insert(Name::new("Plane"));
}

pub fn move_player(keyboard: Res<Input<KeyCode>>, time: Res<Time>, mut player_query: Query<&mut Transform, (With<Player>, Without<Camera3d>)>, camera_query: Query<&Transform, With<Camera3d>>) {
  let camera = camera_query.single();


  for mut player_transform in &mut player_query {
    let mut x = 0.0;
    let mut z = 0.0;

    let mut forward = camera.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let mut left = camera.left();
    left.y = 0.0;
    left = left.normalize();

    if keyboard.pressed(KeyCode::W) {
      z += 1.0;
    }

    if keyboard.pressed(KeyCode::S) {
      z -= 1.0;
    }

    if keyboard.pressed(KeyCode::A) {
      x += 1.0;
    }

    if keyboard.pressed(KeyCode::D) {
      x -= 1.0;
    }

    let left_vec: Vec3 = x * left;
    let forward_vec: Vec3 = z * forward;

    let final_vec = left_vec + forward_vec;

     println!("{:?}", final_vec);

    if final_vec != Vec3::ZERO {
      let current_pos = player_transform.translation.clone();
      player_transform.translation += final_vec.normalize() * 2.0 * time.delta_seconds();
      player_transform.look_at(current_pos + final_vec, Vec3::Y);
    }
  }
}
