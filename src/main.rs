use bevy::prelude::*;

#[derive(Default, Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_input, apply_velocity).chain())
        .add_systems(Update, print_player_info)
        .run();
}

fn apply_velocity(time: Res<Time>, query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query {
        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();
    }
}

fn handle_input(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut velocity: Single<&mut Velocity, With<Player>>,
) {
    let acceleration = 10.0 * time.delta_secs();
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::camera::ScalingMode::FixedVertical {
                viewport_height: 10.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
    commands.spawn((
        Player,
        Transform::default().with_scale(Vec3::splat(0.5)),
        Velocity::default(),
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::from(bevy::color::palettes::basic::GREEN))),
    ));
}

fn print_player_info(player: Single<(&Transform, &Velocity), With<Player>>) {
    let (transform, velocity) = player.into_inner();
    println!(
        "Player: Position {:.2} {:.2}, Velocity {:.2} {:.2}",
        transform.translation.x, transform.translation.y, velocity.0.x, velocity.0.y
    );
}
