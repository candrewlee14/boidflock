pub use ggez::{
    graphics::{self, spritebatch::SpriteBatch, DrawParam},
    Context, GameResult,
};
pub use glam::Vec2;
pub use rand::prelude::*;
pub use rand::Rng;
pub use std::collections::HashSet;

pub type Point2 = Vec2;
pub type Vector2 = Vec2;

pub const COHERENCE: f32 = 0.0005;
pub const SEPARATION: f32 = 0.005;
pub const ALIGNMENT: f32 = 0.05;
pub const AVOID_RANGE: f32 = 20.;
pub const VISUAL_RANGE: f32 = 50.;
pub const MAX_VELOC: f32 = 5.;
pub const MAX_NEIGHBORS: usize = 8;
pub const EDGE_TURN_MARGIN: f32 = 20.;
pub const TURN_FACTOR: f32 = 1.;
