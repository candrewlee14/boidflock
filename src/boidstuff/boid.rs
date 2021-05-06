use super::constants::*;
use crate::cliargs::BoidSimOpt;
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
            self.vel += (center - self.pos) * opt.COHERENCE
        };
    }
    pub fn avoid_other_boids(&mut self, closest: &Vec<Boid>, opt: &BoidSimOpt) {
        let mut delta = Vector2::new(0., 0.);
        for neighbor in closest {
            if self.distance_to(neighbor) < opt.AVOID_RANGE {
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
    pub fn random_vel_change(&mut self, rng: &mut ThreadRng, opt: &BoidSimOpt) {
        let angle = (rng.gen::<f32>() - 0.5) * opt.MAX_RAND_ROTATE;
        let rot_matr =
            glam::Mat2::from_cols_array(&[angle.cos(), angle.sin(), -angle.sin(), angle.cos()]);
        self.vel = rot_matr * self.vel;
    }
    pub fn distance_to(&self, other: &Self) -> f32 {
        self.pos.distance(other.pos)
    }
    pub fn in_sight_range(&self, other: &Self, opt: &BoidSimOpt) -> bool {
        if self.distance_to(other) < opt.VISUAL_RANGE {
            let pi2 = 2. * std::f32::consts::PI;
            let angle = self.vel.angle_between(other.pos - self.pos);
            if angle >= -opt.SIGHT_ANGLE / 2. && angle <= opt.SIGHT_ANGLE / 2. {
                return true;
            } else {
                return false;
            }
        }
        false
    }
    pub fn get_dist_if_in_sight(&self, other: &Self, opt: &BoidSimOpt) -> Option<f32> {
        let dist = self.distance_to(other);
        if dist < opt.VISUAL_RANGE {
            let pi2 = 2. * std::f32::consts::PI;
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
        let hslcol = Hsl::new(
            RgbHue::from_radians(self.facing_angle()),
            self.min_dist / 30.,
            0.5,
        );
        let rgbcol: Rgb = hslcol.into();
        Color::new(rgbcol.red, rgbcol.green, rgbcol.blue, 1.)
    }
    pub fn get_drawparam(&self, opt: &BoidSimOpt) -> DrawParam {
        graphics::DrawParam::new()
            .dest(self.pos)
            .rotation(self.facing_angle())
            .offset(Point2::new(0.5, 0.5))
            .scale(Vec2::new(opt.IMG_SCALE, opt.IMG_SCALE))
            .color(self.get_color())
    }
    pub fn keep_within_bounds(&mut self, width: f32, height: f32, opt: &BoidSimOpt) {
        if self.pos[0] < opt.EDGE_TURN_MARGIN {
            self.vel = self
                .vel
                .lerp(Vector2::new(opt.MIN_VELOC, 0.), opt.EDGE_TURN_FACTOR);
        } else if self.pos[0] > width - opt.EDGE_TURN_MARGIN {
            self.vel = self
                .vel
                .lerp(Vector2::new(-opt.MIN_VELOC, 0.), opt.EDGE_TURN_FACTOR);
        }
        if self.pos[1] < opt.EDGE_TURN_MARGIN {
            self.vel = self
                .vel
                .lerp(Vector2::new(0., opt.MIN_VELOC), opt.EDGE_TURN_FACTOR);
        } else if self.pos[1] > height - opt.EDGE_TURN_MARGIN {
            self.vel = self
                .vel
                .lerp(Vector2::new(0., -opt.MIN_VELOC), opt.EDGE_TURN_FACTOR);
        }
    }
}
