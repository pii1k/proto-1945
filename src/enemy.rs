use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::PI;
use crate::consts::*;
use crate::game_state::GameState;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
struct EnemyShooter {
    timer: Timer,
}

#[derive(Component)]
pub struct EnemyBullet;

#[derive(Clone, Copy)]
enum MotionKind {
    Straight,
    Sine,    
    ZigZag,  
}

#[derive(Component)]
struct EnemyMotion {
    kind: MotionKind,
    base_x: f32,   
    amp: f32,      
    period: f32,   
    t: f32,        
}

#[derive(Resource)]
pub struct EnemySpawnTimer { pub timer: Timer }

#[derive(Resource, Default)]
struct WaveState { wave_idx: u32 }

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnTimer {
                timer: Timer::from_seconds(ENEMY_SPAWN_SEC, TimerMode::Repeating),
            })
            .init_resource::<WaveState>()
            .add_systems(
                Update,
                (
                    spawn_enemy_waves,      
                    move_and_gc_enemies,    
                    enemy_fire_bullets,     
                    move_and_gc_enemy_bullets,
                )
                .run_if(in_state(GameState::Playing)),
            );
    }
}


fn spawn_enemy_waves(
    time: Res<Time>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    mut wave: ResMut<WaveState>,
    mut commands: Commands,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.finished() { return; }


    let pattern = match wave.wave_idx % 3 {
        0 => MotionKind::Straight,
        1 => MotionKind::Sine,
        _ => MotionKind::ZigZag,
    };
    wave.wave_idx = wave.wave_idx.wrapping_add(1);


    let half_w = WIN_W * 0.5;
    let y = (WIN_H * 0.5) + ENEMY_SIZE.y;
    for i in 0..ENEMY_WAVE_COUNT {
        let t = if ENEMY_WAVE_COUNT == 1 {
            0.5
        } else {
            i as f32 / (ENEMY_WAVE_COUNT - 1) as f32
        };
    
        let margin_x = ENEMY_SIZE.x * 0.8;
        let x = lerp(-half_w + margin_x, half_w - margin_x, t);

        spawn_one_enemy(&mut commands, Vec3::new(x, y, 0.0), pattern);
    }

    spawn_timer.timer.reset();
}

fn spawn_one_enemy(commands: &mut Commands, pos: Vec3, kind: MotionKind) {

    let mut rng = rand::rng();
    let mut shooter = EnemyShooter {
        timer: Timer::from_seconds(ENEMY_FIRE_COOLDOWN_SEC, TimerMode::Repeating),
    };
    let jitter = rng.random_range(0.0..ENEMY_FIRE_COOLDOWN_SEC);
    shooter.timer.set_elapsed(std::time::Duration::from_secs_f32(jitter));


    let motion = EnemyMotion {
        kind,
        base_x: pos.x,
        amp: ENEMY_PATTERN_AMP,
        period: ENEMY_PATTERN_PERIOD,
        t: 0.0,
    };

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.9, 0.2, 0.2),
                custom_size: Some(ENEMY_SIZE),
                ..default()
            },
            transform: Transform::from_translation(pos),
            ..default()
        },
        Enemy,
        shooter,
        motion,
    ));
}


fn move_and_gc_enemies(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut Transform, &mut EnemyMotion), With<Enemy>>,
) {
    let dt = time.delta_seconds();
    let half_w = WIN_W * 0.5;
    let half_h = WIN_H * 0.5;

    for (e, mut t, mut m) in &mut q {
        m.t += dt;

    
        t.translation.y -= ENEMY_SPEED * dt;

    
        let x_off = match m.kind {
            MotionKind::Straight => 0.0,
            MotionKind::Sine => {
                let phase = (2.0 * PI) * (m.t / m.period);
                m.amp * phase.sin()
            }
            MotionKind::ZigZag => {
            
                let phase = (2.0 * PI) * (m.t / m.period);
                let tri = (2.0 / PI) * (phase.sin()).asin();
                m.amp * tri
            }
        };
        t.translation.x = (m.base_x + x_off)
            .clamp(-half_w + ENEMY_SIZE.x * 0.5, half_w - ENEMY_SIZE.x * 0.5);

    
        if t.translation.y + ENEMY_SIZE.y < -half_h {
            commands.entity(e).despawn();
        }
    }
}


fn enemy_fire_bullets(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(&Transform, &mut EnemyShooter)>,
) {
    for (t, mut shooter) in &mut q {
        shooter.timer.tick(time.delta());
        if shooter.timer.finished() {
            let muzzle_y = t.translation.y - ENEMY_SIZE.y * 0.5 - ENEMY_BULLET_SIZE.y * 0.5 - 2.0;

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(1.0, 0.9, 0.2),
                        custom_size: Some(ENEMY_BULLET_SIZE),
                        ..default()
                    },
                    transform: Transform::from_xyz(t.translation.x, muzzle_y, 0.0),
                    ..default()
                },
                EnemyBullet,
            ));

            shooter.timer.reset();
        }
    }
}

fn move_and_gc_enemy_bullets(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut Transform), With<EnemyBullet>>,
) {
    let half_h = WIN_H * 0.5;
    for (e, mut t) in &mut q {
        t.translation.y -= ENEMY_BULLET_SPEED * time.delta_seconds();
        if t.translation.y + ENEMY_BULLET_SIZE.y < -half_h {
            commands.entity(e).despawn();
        }
    }
}

#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 { a + (b - a) * t }
