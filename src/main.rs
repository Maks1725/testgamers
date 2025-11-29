use bevy::color::palettes::basic::*;
use bevy::prelude::*;
use bevy::window::{CursorIcon, CustomCursor, CustomCursorImage};

#[derive(Default, Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerWeapon;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_environment)
        .add_systems(Startup, setup_cursor)
        .add_systems(Update, (handle_keyboard_input, apply_velocity).chain())
        .add_systems(Update, rotate_player_weapon)
        .add_systems(Update, move_camera)
        .add_systems(Update, print_player_info)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2d,
        Camera::default(),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::camera::ScalingMode::FixedVertical {
                viewport_height: 10.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
    commands.spawn((
        Player,
        Transform::default()
            .with_scale(Vec3::splat(0.5))
            .with_translation(Vec3::default().with_z(1.0)),
        Velocity::default(),
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::from(GRAY))),
        children![(
            PlayerWeapon,
            Transform::default()
                .with_scale(vec3(0.2, 0.8, 1.0))
                .with_translation(vec3(0.0, 0.0, 2.0)),
            Mesh2d(meshes.add(Rectangle::default())),
            MeshMaterial2d(materials.add(Color::from(BLACK))),
        )],
    ));
}

fn setup_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Transform::default()
            .with_scale(Vec3::splat(8.0))
            .with_translation(Vec3::default().with_z(0.0)),
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(GREEN))),
    ));
}

fn setup_cursor(
    mut commands: Commands,
    window: Single<Entity, With<Window>>,
    asset_server: Res<AssetServer>,
) {
    let cursor = asset_server.load("sight.png");
    commands
        .entity(*window)
        .insert(CursorIcon::Custom(CustomCursor::Image(CustomCursorImage {
            handle: cursor,
            hotspot: (7, 7),
            ..Default::default()
        })));
}

fn apply_velocity(time: Res<Time>, query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query {
        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();
    }
}

fn handle_keyboard_input(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut velocity: Single<&mut Velocity, With<Player>>,
) {
    let acceleration = 20.0 * time.delta_secs();
    let mut speed = 2.0;
    let mut target_velocity = Vec2::ZERO;

    if input.pressed(KeyCode::ShiftLeft) {
        speed = 4.0;
    }

    if input.pressed(KeyCode::KeyW) {
        target_velocity.y += 1.0;
    };

    if input.pressed(KeyCode::KeyS) {
        target_velocity.y -= 1.0;
    };

    if input.pressed(KeyCode::KeyA) {
        target_velocity.x -= 1.0;
    };

    if input.pressed(KeyCode::KeyD) {
        target_velocity.x += 1.0;
    };

    if target_velocity.length_squared() > 0.0 {
        target_velocity = target_velocity.normalize() * speed;
    }
    target_velocity = (target_velocity - velocity.0).clamp_length_max(acceleration);
    velocity.0 += target_velocity;
}

fn move_camera(
    player: Single<&Transform, With<Player>>,
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    camera.translation = camera.translation.lerp(direction, 0.2);
}

fn rotate_player_weapon(
    mut transform: Single<&mut Transform, (With<PlayerWeapon>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<PlayerWeapon>)>,
    query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
) {
    let (camera, camera_transform) = query.into_inner();
    if let Some(cursor_position) = window.cursor_position()
        && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position)
    {
        let player_translation = player.translation.xy();
        let to_cursor = (world_pos - player_translation).normalize();
        let rotation = Quat::from_rotation_arc(Vec3::Y, to_cursor.extend(0.0));
        transform.rotation = rotation;
    }
}

fn print_player_info(player: Single<(&Transform, &Velocity), With<Player>>) {
    let (transform, velocity) = player.into_inner();
    println!(
        "Player: Position {:.2} {:.2}, Velocity {:.2} {:.2}",
        transform.translation.x, transform.translation.y, velocity.0.x, velocity.0.y
    );
}
