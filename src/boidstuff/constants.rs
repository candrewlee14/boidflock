pub use ggez::{
    graphics::{self, spritebatch::SpriteBatch, DrawParam, Color},
    Context, GameResult,
};
pub use glam::Vec2;
pub use rand::prelude::*;
pub use rand::Rng;
pub use std::collections::HashSet;

pub type Point2 = Vec2;
pub type Vector2 = Vec2;

pub const COHERENCE: f32 = 0.015;
pub const SEPARATION: f32 = 0.045;
pub const ALIGNMENT: f32 = 0.25;
pub const AVOID_RANGE: f32 = 34.;
pub const VISUAL_RANGE: f32 = 50.;
pub const MAX_VELOC: f32 = 5.;
pub const MIN_VELOC: f32 = 2.;
pub const MAX_NEIGHBORS: usize = 20;
pub const EDGE_TURN_MARGIN: f32 = 20.;
pub const TURN_FACTOR: f32 = 1.9;
pub const IMG_SCALE: f32 = 0.2;
pub const COLOR_DIVISOR: f32 = 500.;
pub const MAX_RAND_CHANGE: f32 = 0.2;