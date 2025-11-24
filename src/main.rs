use bevy::prelude::*;

#[derive(Default, Component)]
struct Velocity {
    x: f32,
    y: f32,
}

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
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

fn handle_input(
    input: Res<ButtonInput<KeyCode>>,
    mut velocity: Single<&mut Velocity, With<Player>>,
) {
    let speed = 3.0;
    velocity.x = 0.0;
    velocity.y = 0.0;

    if input.pressed(KeyCode::KeyW) {
        velocity.y += speed;
    };

    if input.pressed(KeyCode::KeyS) {
        velocity.y -= speed;
    };

    if input.pressed(KeyCode::KeyA) {
        velocity.x -= speed;
    };

    if input.pressed(KeyCode::KeyD) {
        velocity.x += speed;
    };
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
                viewport_height: 20.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
    commands.spawn((
        Player,
        Transform::default().with_scale(Vec3::splat(0.8)),
        Velocity::default(),
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(bevy::color::palettes::basic::WHITE))),
    ));
}

fn print_player_info(query: Single<(&Transform, &Velocity), With<Player>>) {
    let (transform, velocity) = (query.0, query.1);
    println!(
        "Player: Position {:.2} {:.2}, Velocity {:.2} {:.2}",
        transform.translation.x, transform.translation.y, velocity.x, velocity.y
    );
}
