use bevy::prelude::*;
use crate::{consts::*, bullet::PlayerBullet, enemy::{Enemy, EnemyBullet}, player::Player};
use crate::game_state::GameState;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                bullet_enemy_collisions,
                player_enemy_collisions,
                player_enemybullet_collisions,
            )
            .run_if(in_state(GameState::Playing)),
        );
    }
}

fn bullet_enemy_collisions(
    mut commands: Commands,
    q_bullets: Query<(Entity, &Transform), With<PlayerBullet>>,
    q_enemies: Query<(Entity, &Transform), With<Enemy>>,
) {
    let half_b = BULLET_SIZE * 0.5;
    let half_e = ENEMY_SIZE * 0.5;

    for (b_ent, b_t) in &q_bullets {
        let b_pos = b_t.translation.truncate();
        for (e_ent, e_t) in &q_enemies {
            let e_pos = e_t.translation.truncate();
            if aabb_overlap(b_pos, half_b, e_pos, half_e) {
                commands.entity(b_ent).despawn();
                commands.entity(e_ent).despawn();
                break;
            }
        }
    }
}

fn player_enemy_collisions(
    mut commands: Commands,
    q_player: Query<(Entity, &Transform), With<Player>>,
    q_enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok((p_ent, p_t)) = q_player.get_single() {
        let p_pos = p_t.translation.truncate();
        let half_p = Vec2::splat(PLAYER_RADIUS);
        let half_e = ENEMY_SIZE * 0.5;

        for (e_ent, e_t) in &q_enemies {
            let e_pos = e_t.translation.truncate();
            if aabb_overlap(p_pos, half_p, e_pos, half_e) {
                commands.entity(p_ent).despawn();
                commands.entity(e_ent).despawn();
                next_state.set(GameState::GameOver);
                break;
            }
        }
    }
}

fn player_enemybullet_collisions(
    mut commands: Commands,
    q_player: Query<(Entity, &Transform), With<Player>>,
    q_enemy_bullets: Query<(Entity, &Transform), With<EnemyBullet>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok((p_ent, p_t)) = q_player.get_single() {
        let p_pos = p_t.translation.truncate();
        let half_p = Vec2::splat(PLAYER_RADIUS);
        let half_b = ENEMY_BULLET_SIZE * 0.5;

        for (b_ent, b_t) in &q_enemy_bullets {
            let b_pos = b_t.translation.truncate();
            if aabb_overlap(p_pos, half_p, b_pos, half_b) {
                commands.entity(p_ent).despawn();
                commands.entity(b_ent).despawn();
                next_state.set(GameState::GameOver);
                break;
            }
        }
    }
}

#[inline]
fn aabb_overlap(a_center: Vec2, a_half: Vec2, b_center: Vec2, b_half: Vec2) -> bool {
    (a_center.x - b_center.x).abs() <= (a_half.x + b_half.x) &&
    (a_center.y - b_center.y).abs() <= (a_half.y + b_half.y)
}
