pub use ggez::{
    graphics::{self, spritebatch::SpriteBatch, Color, DrawParam},
    Context, GameResult,
};
pub use glam::Vec2;
pub use rand::prelude::*;
use rand_xoshiro::Xoroshiro128Plus;
pub use std::collections::HashSet;

pub type Vector2 = Vec2;
use crate::boid::{get_cell_for_point, Boid, Point2};
use crate::cliargs::BoidSimOpt;

fn rand_vec2(rng: &mut Xoroshiro128Plus, max_1: f32, max_2: f32, centered: bool) -> Vec2 {
    let mut x1 = rng.gen::<f32>();
    let mut x2 = rng.gen::<f32>();
    if centered {
        x1 -= 0.5;
        x2 -= 0.5;
    }
    let final1 = x1 * max_1;
    let final2 = x2 * max_2;
    Vec2::new(final1, final2)
}

pub struct BoidCloud {
    pub width: f32,
    pub height: f32,
    pub boid_cells: Vec<Vec<Boid>>,
    pub boid_count: usize,
    pub opt: BoidSimOpt,
}
impl BoidCloud {
    pub fn new(
        boid_count: usize,
        width: f32,
        height: f32,
        rng: &mut Xoroshiro128Plus,
        opt: BoidSimOpt,
    ) -> Self {
        let mut boid_cells: Vec<Vec<Boid>> = vec![
            Vec::<Boid>::with_capacity(opt.CUR_CELL_NEIGHBORS);
            ((width / opt.VISUAL_RANGE).ceil() * (height / opt.VISUAL_RANGE).ceil())
                as usize
        ];
        (0..boid_count)
            .map(|_| {
                Boid::new(
                    rand_vec2(rng, width, height, false),
                    rand_vec2(rng, opt.MAX_VELOC / 2., opt.MAX_VELOC / 2., true),
                )
            })
            .for_each(|boid| boid_cells[boid.get_cell(width, height, &opt)].push(boid));
        Self {
            width,
            height,
            boid_cells: boid_cells,
            boid_count,
            opt,
        }
    }

    pub fn update(&mut self, rng: &mut Xoroshiro128Plus) {
        for cell_num in 0..self.boid_cells.len() {
            let mut i = 0;
            while i < self.boid_cells[cell_num].len() {
                let mut boid = self.boid_cells[cell_num][i];
                let forward_cell = get_cell_for_point(
                    boid.pos + boid.vel.normalize() * self.opt.VISUAL_RANGE,
                    self.width,
                    self.height,
                    &self.opt,
                );
                let mut close_boids: Vec<Boid> = Vec::new();
                let mut close_boids_chain = self.boid_cells[cell_num]
                    .iter()
                    .filter(|other| {
                        let _pi2 = 2. * std::f32::consts::PI;
                        let angle = boid.vel.angle_between(other.pos - boid.pos);
                        return angle >= -self.opt.SIGHT_ANGLE / 2.
                            && angle <= self.opt.SIGHT_ANGLE / 2.;
                    })
                    .take(self.opt.CUR_CELL_NEIGHBORS)
                    .chain(
                        self.boid_cells[forward_cell]
                            .iter()
                            .filter(|other| {
                                let _pi2 = 2. * std::f32::consts::PI;
                                let angle = boid.vel.angle_between(other.pos - boid.pos);
                                return angle >= -self.opt.SIGHT_ANGLE / 2.
                                    && angle <= self.opt.SIGHT_ANGLE / 2.;
                            })
                            .take(self.opt.FORWARD_CELL_NEIGHBORS),
                    );
                close_boids.extend(close_boids_chain);
                let angle = self.opt.SIGHT_ANGLE / 2.;
                let rot_matr = glam::Mat2::from_cols_array(&[
                    angle.cos(),
                    angle.sin(),
                    -angle.sin(),
                    angle.cos(),
                ]);
                let side_cell = get_cell_for_point(
                    boid.pos + rot_matr * boid.vel.normalize() * self.opt.VISUAL_RANGE,
                    self.width,
                    self.height,
                    &self.opt,
                )
                .clamp(0, self.boid_cells.len() - 1);
                if side_cell != forward_cell && side_cell != i {
                    let side_cell_filtered = self.boid_cells[side_cell]
                        .iter()
                        .filter(|other| {
                            let _pi2 = 2. * std::f32::consts::PI;
                            let angle = boid.vel.angle_between(other.pos - boid.pos);
                            return angle >= -self.opt.SIGHT_ANGLE / 2.
                                && angle <= self.opt.SIGHT_ANGLE / 2.;
                        })
                        .take(self.opt.FORWARD_CELL_NEIGHBORS);
                    close_boids.extend(side_cell_filtered);
                }
                let left_angle = -self.opt.SIGHT_ANGLE / 2.;
                let left_rot_matr = glam::Mat2::from_cols_array(&[
                    angle.cos(),
                    angle.sin(),
                    -angle.sin(),
                    angle.cos(),
                ]);
                let left_side_cell = get_cell_for_point(
                    boid.pos + rot_matr * boid.vel.normalize() * self.opt.VISUAL_RANGE,
                    self.width,
                    self.height,
                    &self.opt,
                )
                .clamp(0, self.boid_cells.len() - 1);
                if left_side_cell != side_cell
                    && left_side_cell != forward_cell
                    && left_side_cell != i
                {
                    let left_side_cell_filtered = self.boid_cells[left_side_cell]
                        .iter()
                        .filter(|other| {
                            let _pi2 = 2. * std::f32::consts::PI;
                            let angle = boid.vel.angle_between(other.pos - boid.pos);
                            return angle >= -self.opt.SIGHT_ANGLE / 2.
                                && angle <= self.opt.SIGHT_ANGLE / 2.;
                        })
                        .take(self.opt.FORWARD_CELL_NEIGHBORS);
                    close_boids.extend(left_side_cell_filtered);
                }
                boid.fly_towards_center(&close_boids, &self.opt);
                boid.avoid_other_boids(&close_boids, &self.opt);
                boid.match_velocities(&close_boids, &self.opt);
                boid.random_vel_change(rng, &self.opt);
                boid.limit_speed(&self.opt);
                boid.keep_within_bounds(self.width, self.height, &self.opt);
                boid.update_pos();
                self.boid_cells[cell_num][i] = boid;
                let correct_cell = self.boid_cells[cell_num][i]
                    .get_cell(self.width, self.height, &self.opt)
                    .clamp(0, self.boid_cells.len() - 1);
                if correct_cell != cell_num {
                    let boid = self.boid_cells[cell_num].swap_remove(i);
                    self.boid_cells[correct_cell].push(boid);
                } else {
                    i += 1;
                }
            }
        }
    }

    pub fn add_boids_to_spritebatch(&self, img_batch: &mut SpriteBatch) {
        for boid in self.boid_cells.iter().flatten() {
            img_batch.add(boid.get_drawparam(&self.opt));
        }
    }
}
