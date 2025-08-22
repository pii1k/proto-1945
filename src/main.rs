mod consts;
mod player;
mod bullet;
mod enemy;
mod collision;
mod game_state; // ← 추가

use bevy::prelude::*;
use bevy::math::primitives::{RegularPolygon, Circle};              // 재스폰용
use bevy::sprite::MaterialMesh2dBundle;                             // 재스폰용
use crate::consts::*;
use crate::player::{Player, PlayerPlugin};
use crate::bullet::{Bullet, BulletPlugin, FireState};
use crate::enemy::{Enemy, EnemyPlugin, EnemySpawnTimer};
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
    q_bullets: Query<Entity, With<Bullet>>,
    q_enemies: Query<Entity, With<Enemy>>,
    q_players: Query<Entity, With<Player>>,
    mut fire: ResMut<FireState>,
    mut enemy_timer: ResMut<EnemySpawnTimer>,
) {
    if !kb.just_pressed(KeyCode::KeyR) { return; }

    // 1) 월드 정리
    for e in q_bullets.iter() { commands.entity(e).despawn(); }
    for e in q_enemies.iter() { commands.entity(e).despawn(); }
    for e in q_players.iter() { commands.entity(e).despawn(); }

    // 2) 플레이어 재스폰 (player.rs와 동일 로직)
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

    // 3) 타이머 리셋/프라임
    fire.timer.reset();
    let d = fire.timer.duration();
    fire.timer.set_elapsed(d); // 재시작 직후 홀드 시 즉시 발사
    enemy_timer.timer.reset();

    // 4) 상태 전환
    next.set(GameState::Playing);
}