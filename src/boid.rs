pub use ggez::{
    graphics::{self, spritebatch::SpriteBatch, Color, DrawParam},
    Context, GameResult,
};
pub use glam::Vec2;
pub use rand::prelude::*;
pub use rand_xoshiro::Xoroshiro128Plus;
pub use std::collections::HashSet;

pub type Point2 = Vec2;
pub type Vector2 = Vec2;
use super::cliargs::BoidSimOpt;
use palette::{rgb::Rgb, Hsl, RgbHue};

#[derive(Debug, Clone, Copy)]
pub struct Boid {
    /// (x, y) position of Boid
    pub pos: Point2,
    /// (x, y) velocity of Boid
    pub vel: Vector2,
}
pub fn get_cell_for_point(pos: Point2, width: f32, height: f32, opt: &BoidSimOpt) -> usize {
    ((pos[1].clamp(0., height - 1.) / opt.VISUAL_RANGE).floor()
        * (width / opt.VISUAL_RANGE).floor()
        + (pos[0].clamp(0., width - 1.) / opt.VISUAL_RANGE).floor()) as usize
}

impl Boid {
    pub fn new(pos: Point2, vel: Vector2) -> Self {
        Self { pos, vel }
    }
    pub fn get_cell(&self, width: f32, height: f32, opt: &BoidSimOpt) -> usize {
        get_cell_for_point(self.pos, width, height, opt)
    }
    pub fn fly_towards_center(&mut self, closest: &Vec<Boid>, opt: &BoidSimOpt) {
        let mut neighbor_count: f32 = 0.;
        let mut center = Vector2::ZERO;
        for neighbor in closest {
            if self.in_sight_range(neighbor, &opt) {
                center += neighbor.pos;
                neighbor_count += 1.;
            }
        }
        if neighbor_count > 0. {
            center /= neighbor_count;
            self.vel += (center - self.pos) * opt.COHERENCE;
        }
    }
    pub fn avoid_other_boids(&mut self, closest: &Vec<Boid>, opt: &BoidSimOpt) {
        let mut delta = Vector2::new(0., 0.);
        for neighbor in closest {
            if self.sq_distance_to(neighbor) < opt.AVOID_RANGE.powi(2) {
                delta += self.pos - neighbor.pos;
            }
        }
        self.vel += delta * opt.SEPARATION;
    }
    pub fn match_velocities(&mut self, closest: &Vec<Boid>, opt: &BoidSimOpt) {
        let mut neighbor_count: f32 = 0.;
        let mut avg_vel = Vector2::ZERO;
        for neighbor in closest {
            if self.in_sight_range(neighbor, &opt) {
                avg_vel += neighbor.vel;
                neighbor_count += 1.;
            }
        }
        if neighbor_count > 0. {
            avg_vel /= neighbor_count;
            self.vel += (avg_vel - self.vel) * opt.ALIGNMENT;
        }
    }
    pub fn random_vel_change(&mut self, rng: &mut Xoroshiro128Plus, opt: &BoidSimOpt) {
        let angle = (rng.gen::<f32>() - 0.5) * opt.MAX_RAND_ROTATE;
        let rot_matr =
            glam::Mat2::from_cols_array(&[angle.cos(), angle.sin(), -angle.sin(), angle.cos()]);
        self.vel = rot_matr * self.vel;
    }
    pub fn distance_to(&self, other: &Self) -> f32 {
        self.pos.distance(other.pos)
    }
    pub fn in_sight_range(&self, other: &Self, opt: &BoidSimOpt) -> bool {
        if self.sq_distance_to(other) < opt.VISUAL_RANGE.powi(2) {
            let _pi2 = 2. * std::f32::consts::PI;
            let angle = self.vel.angle_between(other.pos - self.pos);
            return angle >= -opt.SIGHT_ANGLE / 2. && angle <= opt.SIGHT_ANGLE / 2.;
        }
        false
    }
    pub fn get_dist_if_in_sight(&self, other: &Self, opt: &BoidSimOpt) -> Option<f32> {
        let dist = self.distance_to(other);
        if dist < opt.VISUAL_RANGE {
            let _pi2 = 2. * std::f32::consts::PI;
            let angle = self.vel.angle_between(other.pos - self.pos);
            if angle >= -opt.SIGHT_ANGLE / 2. && angle <= opt.SIGHT_ANGLE / 2. {
                return Some(dist);
            }
        }
        None
    }
    pub fn sq_distance_to(&self, other: &Self) -> f32 {
        self.pos.distance_squared(other.pos)
    }
    pub fn limit_speed(&mut self, opt: &BoidSimOpt) {
        let norm = self.vel.length();
        if norm > opt.MAX_VELOC {
            self.vel = self.vel.normalize() * opt.MAX_VELOC;
        } else if norm < opt.MIN_VELOC {
            self.vel = self.vel.normalize() * opt.MIN_VELOC;
        }
    }
    pub fn update_pos(&mut self) {
        self.pos += self.vel
    }
    pub fn facing_angle(&self) -> f32 {
        let facing = self.vel[1].atan2(self.vel[0]) + 0.5 * std::f32::consts::PI;
        facing
    }
    fn get_color(&self) -> Color {
        let hslcol = Hsl::new(RgbHue::from_radians(self.facing_angle()), 1., 0.5);
        let rgbcol: Rgb = hslcol.into();
        Color::new(rgbcol.red, rgbcol.green, rgbcol.blue, 1.)
    }
    pub fn get_drawparam(&self, opt: &BoidSimOpt) -> DrawParam {
        graphics::DrawParam::new()
            .dest(self.pos * opt.ZOOM_SCALE)
            .rotation(self.facing_angle())
            .offset(Point2::new(0.5, 0.5))
            .scale(Vec2::new(
                opt.IMG_SCALE * opt.ZOOM_SCALE,
                opt.IMG_SCALE * opt.ZOOM_SCALE,
            ))
            .color(self.get_color())
    }
    pub fn keep_within_bounds(&mut self, width: f32, height: f32, opt: &BoidSimOpt) {
        let pixel_margin = opt.EDGE_TURN_MARGIN / opt.ZOOM_SCALE;
        if self.pos[0] < pixel_margin {
            self.vel = self
                .vel
                .lerp(Vector2::new(opt.MIN_VELOC, 0.), opt.EDGE_TURN_FACTOR);
        } else if self.pos[0] > width - pixel_margin {
            self.vel = self
                .vel
                .lerp(Vector2::new(-opt.MIN_VELOC, 0.), opt.EDGE_TURN_FACTOR);
        }
        if self.pos[1] < pixel_margin {
            self.vel = self
                .vel
                .lerp(Vector2::new(0., opt.MIN_VELOC), opt.EDGE_TURN_FACTOR);
        } else if self.pos[1] > height - pixel_margin {
            self.vel = self
                .vel
                .lerp(Vector2::new(0., -opt.MIN_VELOC), opt.EDGE_TURN_FACTOR);
        }
    }
}
