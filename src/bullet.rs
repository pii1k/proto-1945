use bevy::prelude::*;
use crate::{consts::*, player::Player};
use crate::game_state::GameState;

#[derive(Component)]
pub struct Bullet;

#[derive(Resource)]
pub struct FireState {
    pub timer: Timer,
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FireState {
                timer: Timer::from_seconds(FIRE_COOLDOWN_SEC, TimerMode::Repeating),
            })
            .add_systems(Startup, prime_fire_timer)
            .add_systems(Update, (fire_bullets, move_and_gc_bullets).run_if(in_state(GameState::Playing)));
    }
}

fn prime_fire_timer(mut fire: ResMut<FireState>) {
    let d = fire.timer.duration();   
    fire.timer.set_elapsed(d);       
}

fn fire_bullets(
    time: Res<Time>,
    kb: Res<ButtonInput<KeyCode>>,
    mut fire: ResMut<FireState>,
    mut commands: Commands,
    q_player: Query<&Transform, With<Player>>,
) {

    fire.timer.tick(time.delta());
    let pt = q_player.single();

    if kb.just_pressed(KeyCode::KeyJ) {
        spawn_bullet(&mut commands, pt.translation);
        fire.timer.reset();                   
        return;                               
    }

    if kb.pressed(KeyCode::KeyJ) && fire.timer.finished() {
        spawn_bullet(&mut commands, pt.translation);
        fire.timer.reset();
    }
}

fn spawn_bullet(commands: &mut Commands, player_pos: Vec3) {
    let muzzle_y = player_pos.y + PLAYER_RADIUS + BULLET_SIZE.y * 0.5 + 2.0;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(BULLET_SIZE),
                ..default()
            },
            transform: Transform::from_xyz(player_pos.x, muzzle_y, 0.0),
            ..default()
        },
        Bullet,
    ));
}

fn move_and_gc_bullets(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut Transform), With<Bullet>>,
) {
    let half_h = WIN_H * 0.5;

    for (e, mut t) in &mut q {
        t.translation.y += BULLET_SPEED * time.delta_seconds();

        if t.translation.y - BULLET_SIZE.y > half_h {
            commands.entity(e).despawn();
        }
    }
}
