use bevy::prelude::*;
use bevy::math::primitives::{RegularPolygon, Circle};
use bevy::sprite::MaterialMesh2dBundle;
use crate::consts::*;
use crate::game_state::GameState;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
           .add_systems(Update, move_and_clamp_player.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let triangle = Mesh::from(RegularPolygon {
        sides: 3,
        circumcircle: Circle { radius: PLAYER_RADIUS },
    });

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(triangle).into(),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            transform: Transform::from_xyz(0.0, -WIN_H * 0.4, 0.0),
            ..default()
        },
        Player,
    ));
}

fn move_and_clamp_player(
    time: Res<Time>,
    kb: Res<ButtonInput<KeyCode>>,
    mut q: Query<&mut Transform, With<Player>>,
) {
    let mut t = q.single_mut();

    let mut dir = Vec2::ZERO;
    if kb.pressed(KeyCode::ArrowLeft) || kb.pressed(KeyCode::KeyA) { dir.x -= 1.0; }
    if kb.pressed(KeyCode::ArrowRight) || kb.pressed(KeyCode::KeyD) { dir.x += 1.0; }
    if kb.pressed(KeyCode::ArrowUp) || kb.pressed(KeyCode::KeyW) { dir.y += 1.0; }
    if kb.pressed(KeyCode::ArrowDown) || kb.pressed(KeyCode::KeyS) { dir.y -= 1.0; }

    if dir != Vec2::ZERO {
        t.translation += (dir.normalize() * PLAYER_SPEED * time.delta_seconds()).extend(0.0);
    }

    let half_w = WIN_W * 0.5;
    let half_h = WIN_H * 0.5;
    let margin = PLAYER_RADIUS * t.scale.x.max(t.scale.y);

    t.translation.x = t.translation.x.clamp(-half_w + margin, half_w - margin);
    t.translation.y = t.translation.y.clamp(-half_h + margin, half_h - margin);
}
