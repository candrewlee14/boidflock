pub use ggez::{
    graphics::{self, spritebatch::SpriteBatch, Color, DrawParam},
    Context, GameResult,
};
pub use glam::Vec2;
pub use rand::prelude::*;
use rand_xoshiro::Xoroshiro128Plus;
pub use std::collections::HashSet;
pub use rayon::prelude::*;

pub type Vector2 = Vec2;
use crate::boid::{Boid, Point2, get_cell_for_point};
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
            Vec::<Boid>::with_capacity(opt.MAX_NEIGHBORS);
            ((width / opt.VISUAL_RANGE).ceil() * (height / opt.VISUAL_RANGE).ceil()) as usize
        ];
        (0..boid_count)
            .map(|_| {
                Boid::new(
                    rand_vec2(rng, width - opt.IMG_SCALE, height - opt.EDGE_TURN_MARGIN, false),
                    rand_vec2(rng, opt.MAX_VELOC / 2., opt.MAX_VELOC / 2., true),
                )
            })
            .for_each(|boid| boid_cells[boid.get_cell(width, &opt)].push(boid));
        // println!("{:#?}", boid_cells);

        Self {
            width,
            height,
            boid_cells: boid_cells,
            boid_count,
            opt,
        }
    }
    pub fn update(&mut self, rng: &mut Xoroshiro128Plus) {
        for cell_num in (0..self.boid_cells.len()) {
            let mut i = 0;
            while i < self.boid_cells[cell_num].len() {
                let mut boid =  self.boid_cells[cell_num][i];
                let forward_cell = get_cell_for_point(boid.pos + boid.vel.normalize() * self.opt.VISUAL_RANGE, self.width, &self.opt);
                let mut close_boids_iter = self.boid_cells[cell_num].iter();
                let mut close_boids : Vec<Boid>;
                if forward_cell > 0 && forward_cell < self.boid_cells.len() {
                   close_boids = close_boids_iter.chain(self.boid_cells[forward_cell].iter()).cloned().collect();
                }
                else {
                    close_boids = close_boids_iter.cloned().collect();
                }
                boid.fly_towards_center(&close_boids, &self.opt);
                boid.avoid_other_boids(&close_boids, &self.opt);
                boid.match_velocities(&close_boids, &self.opt);
                boid.random_vel_change(rng, &self.opt);
                boid.limit_speed(&self.opt);
                boid.keep_within_bounds(self.width, self.height, &self.opt);
                boid.update_pos();
                self.boid_cells[cell_num][i] = boid;
                let correct_cell = self.boid_cells[cell_num][i].get_cell(self.width, &self.opt);
                if correct_cell != cell_num {
                    let boid = self.boid_cells[cell_num].swap_remove(i);
                    self.boid_cells[correct_cell].push(boid);
                } else {
                    i += 1;
                }
            }
        }
    }
    // pub fn update(&mut self, rng: &mut Xoroshiro128Plus) {
    //     let opt = self.opt.clone();
    //     let boids = self.boids.clone();
    //     for mut boid in self.boids.iter_mut() {
    //         if self.boid_count >= self.opt.MAX_NEIGHBORS {
    //             let mut dist_and_closest = boids
    //                 .iter()
    //                 .filter_map(|other| {
    //                     if let Some(dist) = boid.get_dist_if_in_sight(other, &opt) {
    //                         return Some((dist, *other));
    //                     }
    //                     None
    //                 })
    //                 .take(self.opt.NEIGHBORS_TO_SEE)
    //                 .collect::<Vec<(f32, Boid)>>();
    //             if dist_and_closest.len() > self.opt.MAX_NEIGHBORS {
    //                 dist_and_closest.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    //             }
    //             let closest: Vec<Boid> = dist_and_closest
    //                 .iter()
    //                 .take(self.opt.MAX_NEIGHBORS)
    //                 .map(|tup| tup.1)
    //                 .collect();
    //             if !closest.is_empty() {
    //                 boid.min_dist = boid.distance_to(&closest[0]);
    //                 boid.fly_towards_center(&closest, &self.opt);
    //                 boid.avoid_other_boids(&closest, &self.opt);
    //                 boid.match_velocities(&closest, &self.opt);
    //             }
    //         }
    //         boid.random_vel_change(rng, &self.opt);
    //         boid.limit_speed(&self.opt);
    //         boid.keep_within_bounds(self.width, self.height, &self.opt);
    //         boid.update_pos();
    //     }
    //}

    pub fn add_boids_to_spritebatch(&self, img_batch: &mut SpriteBatch) {
        for boid in self.boid_cells.iter().flatten() {
            img_batch.add(boid.get_drawparam(&self.opt));
        }
    }
}
