mod consts;
mod player;
mod bullet;
mod enemy;
mod collision;
mod game_state; 

use bevy::prelude::*;
use bevy::math::primitives::{RegularPolygon, Circle};              
use bevy::sprite::MaterialMesh2dBundle;                             
use crate::consts::*;
use crate::player::{Player, PlayerPlugin};
use crate::bullet::{PlayerBullet, BulletPlugin, FireState};
use crate::enemy::{Enemy, EnemyBullet,EnemyPlugin, EnemySpawnTimer};
use crate::collision::CollisionPlugin;
use crate::game_state::GameState;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "1945 - Prototype".into(),
                    resolution: (WIN_W, WIN_H).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
        )
        .init_state::<GameState>()
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(GameState::GameOver), on_enter_game_over)
        .add_systems(Update, restart_on_r.run_if(in_state(GameState::GameOver)))
        .add_plugins((
            PlayerPlugin,
            BulletPlugin,
            EnemyPlugin,
            CollisionPlugin
        ))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn on_enter_game_over() {
    println!("*** GAME OVER ***  Press 'R' to Restart");
}

fn restart_on_r(
    kb: Res<ButtonInput<KeyCode>>,
    mut next: ResMut<NextState<GameState>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_players: Query<Entity, With<Player>>,
    q_player_bullets: Query<Entity, With<PlayerBullet>>,
    q_enemies: Query<Entity, With<Enemy>>,
    q_enemy_bullets: Query<Entity, With<EnemyBullet>>,
    mut fire: ResMut<FireState>,
    mut enemy_timer: ResMut<EnemySpawnTimer>,
) {
    if !kb.just_pressed(KeyCode::KeyR) { return; }

    for e in q_players.iter() { commands.entity(e).despawn(); }
    for e in q_player_bullets.iter() { commands.entity(e).despawn(); }
    for e in q_enemies.iter() { commands.entity(e).despawn(); }
    for e in q_enemy_bullets.iter() { commands.entity(e).despawn(); }

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

    fire.timer.reset();
    let d = fire.timer.duration();
    fire.timer.set_elapsed(d); 
    enemy_timer.timer.reset();

    next.set(GameState::Playing);
}