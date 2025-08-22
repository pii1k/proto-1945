use bevy::prelude::*;

// window size
pub const WIN_W: f32 = 600.0;
pub const WIN_H: f32 = 800.0;

// player constants
pub const PLAYER_SPEED: f32 = 400.0;
pub const PLAYER_RADIUS: f32 = 15.0;

// bullet constants
pub const BULLET_SPEED: f32 = 900.0;
pub const BULLET_SIZE: Vec2 = Vec2::new(6.0, 16.0);
pub const FIRE_COOLDOWN_SEC: f32 = 0.12;

// enemy constants
pub const ENEMY_SIZE: Vec2 = Vec2::new(28.0, 20.0);
pub const ENEMY_SPEED: f32 = 180.0;
pub const ENEMY_SPAWN_SEC: f32 = 0.8;

// enemy bullet constants
pub const ENEMY_FIRE_COOLDOWN_SEC: f32 = 1.2;
pub const ENEMY_BULLET_SPEED: f32 = 420.0;
pub const ENEMY_BULLET_SIZE: Vec2 = Vec2::new(6.0, 14.0);

// enemy pattern constants
pub const ENEMY_PATTERN_AMP: f32 = 120.0;     
pub const ENEMY_PATTERN_PERIOD: f32 = 1.6;   
pub const ENEMY_WAVE_COUNT: usize = 5;        