pub use ggez::{
    graphics::{self, spritebatch::SpriteBatch, Color, DrawParam},
    Context, GameResult,
};
pub use glam::Vec2;
pub use rand::prelude::*;
pub use rand::Rng;
pub use std::collections::HashSet;

pub type Point2 = Vec2;
pub type Vector2 = Vec2;

// pub const COHERENCE: f32 = 0.085;
// pub const SEPARATION: f32 = 0.095;
// pub const ALIGNMENT: f32 = 0.75;
// pub const AVOID_RANGE: f32 = 20.;
// pub const VISUAL_RANGE: f32 = 60.;
// pub const MAX_VELOC: f32 = 5.;
// pub const MIN_VELOC: f32 = 2.;
// pub const MAX_NEIGHBORS: usize = 10;
// pub const NEIGHBORS_TO_SEE: usize = 45;
// pub const EDGE_TURN_MARGIN: f32 = 50.;
// pub const EDGE_TURN_FACTOR: f32 = 0.2;
// pub const IMG_SCALE: f32 = 0.25;
// pub const MAX_RAND_ROTATE: f32 = 0.3;
// pub const SIGHT_ANGLE: f32 = 0.8 * std::f32::consts::PI;
