use bevy::prelude::*;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, apply_velocity)
        .add_systems(Update, print_player_info)
        .run();
}

fn apply_velocity(time: Res<Time>, query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in query {
        position.x += velocity.x * time.delta_secs();
        position.y += velocity.y * time.delta_secs();
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Position { x: 0.0, y: 0.0 },
        Velocity { x: 0.0, y: 1.0 },
    ));
}

fn print_player_info(query: Query<(&Position, &Velocity), With<Player>>) {
    for (position, velocity) in &query {
        println!(
            "Player: Position {:.2} {:.2}, Velocity {:.2} {:.2}",
            position.x, position.y, velocity.x, velocity.y
        );
    }
}
