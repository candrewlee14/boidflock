use super::constants::*;

#[derive(Debug, Clone, Copy)]
pub struct Boid {
    pub pos: Point2,
    pub vel: Vector2,
    pub min_dist: f32,
}

impl Boid {
    pub fn new(pos: Point2, vel: Vector2) -> Self {
        Self { pos, vel, min_dist: 20. }
    }
    pub fn distance_to(&self, other: &Self) -> f32 {
        self.pos.distance(other.pos)
    }
    pub fn sq_distance_to(&self, other: &Self) -> f32 {
        self.pos.distance_squared(other.pos)
    }
    pub fn limit_speed(&mut self) {
        let norm = self.vel.length();
        if norm > MAX_VELOC {
            self.vel = self.vel.normalize() * MAX_VELOC;
        }
        else if norm < MIN_VELOC {
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
    pub fn get_drawparam(&self) -> DrawParam {
        graphics::DrawParam::new()
            .dest(self.pos)
            .rotation(self.facing_angle())
            .offset(Point2::new(0.5, 0.5))
            .scale(Vec2::new(IMG_SCALE, IMG_SCALE))
            .color(Color::new(1. - self.min_dist / COLOR_DIVISOR, 0., self.min_dist / COLOR_DIVISOR, 1.))
    }
    pub fn keep_within_bounds(&mut self, width: f32, height: f32) {
        if self.pos[0] < EDGE_TURN_MARGIN {
            self.vel += Vector2::new(TURN_FACTOR, 0.);
        }
        if self.pos[0] > width - EDGE_TURN_MARGIN {
            self.vel -= Vector2::new(TURN_FACTOR, 0.);
        }
        if self.pos[1] < EDGE_TURN_MARGIN {
            self.vel += Vector2::new(0., TURN_FACTOR);
        }
        if self.pos[1] > height - EDGE_TURN_MARGIN {
            self.vel -= Vector2::new(0., TURN_FACTOR);
        }
    }
}
