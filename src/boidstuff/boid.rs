use super::constants::*;
use palette::{rgb::Rgb, Hsl, IntoColor, RgbHue};

#[derive(Debug, Clone, Copy)]
pub struct Boid {
    pub pos: Point2,
    pub vel: Vector2,
    pub min_dist: f32,
}

impl Boid {
    pub fn new(pos: Point2, vel: Vector2) -> Self {
        Self {
            pos,
            vel,
            min_dist: 20.,
        }
    }
    pub fn distance_to(&self, other: &Self) -> f32 {
        self.pos.distance(other.pos)
    }
    pub fn in_sight_range(&self, other: &Self) -> bool {
        if self.distance_to(other) < VISUAL_RANGE {
            let pi2 = 2. * std::f32::consts::PI;
            let angle = self.vel.angle_between(other.pos - self.pos);
            if angle >= -SIGHT_ANGLE / 2. && angle <= SIGHT_ANGLE / 2. {
                return true;
            } else {
                return false;
            }
        }
        false
    }
    pub fn get_dist_if_in_sight(&self, other: &Self) -> Option<f32> {
        let dist = self.distance_to(other);
        if dist < VISUAL_RANGE {
            let pi2 = 2. * std::f32::consts::PI;
            let angle = self.vel.angle_between(other.pos - self.pos);
            if angle >= -SIGHT_ANGLE / 2. && angle <= SIGHT_ANGLE / 2. {
                return Some(dist);
            }
        }
        None
    }
    pub fn sq_distance_to(&self, other: &Self) -> f32 {
        self.pos.distance_squared(other.pos)
    }
    pub fn limit_speed(&mut self) {
        let norm = self.vel.length();
        if norm > MAX_VELOC {
            self.vel = self.vel.normalize() * MAX_VELOC;
        } else if norm < MIN_VELOC {
            self.vel = self.vel.normalize() * MIN_VELOC;
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
        //Color::new(
        //     1. - self.facing_angle() / (2. * std::f32::consts::PI),
        //     self.facing_angle().cos(),
        //     self.facing_angle() / (2. * std::f32::consts::PI),
        //     1.,
        // )
        //Color::new(1. - self.min_dist / COLOR_DIVISOR, 0., self.min_dist / COLOR_DIVISOR, 1.)
        let hslcol = Hsl::new(
            RgbHue::from_radians(self.facing_angle()),
            //(self.vel.length() - MIN_VELOC + 1.) / (MAX_VELOC - MIN_VELOC),
            self.min_dist / COLOR_DIVISOR,
            0.5,
        );
        let rgbcol: Rgb = hslcol.into();
        Color::new(rgbcol.red, rgbcol.green, rgbcol.blue, 1.)
    }
    pub fn get_drawparam(&self) -> DrawParam {
        graphics::DrawParam::new()
            .dest(self.pos)
            .rotation(self.facing_angle())
            .offset(Point2::new(0.5, 0.5))
            .scale(Vec2::new(IMG_SCALE, IMG_SCALE))
            .color(self.get_color())
    }
    pub fn keep_within_bounds(&mut self, width: f32, height: f32) {
        if self.pos[0] < EDGE_TURN_MARGIN {
            self.vel = self.vel.lerp(Vector2::new(MIN_VELOC, 0.), TURN_FACTOR);
        } else if self.pos[0] > width - EDGE_TURN_MARGIN {
            self.vel = self.vel.lerp(Vector2::new(-MIN_VELOC, 0.), TURN_FACTOR);
        }
        if self.pos[1] < EDGE_TURN_MARGIN {
            self.vel = self.vel.lerp(Vector2::new(0., MIN_VELOC), TURN_FACTOR);
        } else if self.pos[1] > height - EDGE_TURN_MARGIN {
            self.vel = self.vel.lerp(Vector2::new(0., -MIN_VELOC), TURN_FACTOR);
        }
    }
}
