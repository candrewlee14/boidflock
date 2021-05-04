use super::assets::Assets;
use ggez::{
    graphics::{self, spritebatch::SpriteBatch, DrawParam},
    Context, GameResult,
};
use glam::Vec2;
use rand::prelude::*;
use rand::Rng;
use std::collections::HashSet;

type Point2 = Vec2;
type Vector2 = Vec2;

const COHERENCE: f32 = 0.0005;
const SEPARATION: f32 = 0.005;
const ALIGNMENT: f32 = 0.05;
const AVOID_RANGE: f32 = 20.;
const VISUAL_RANGE: f32 = 50.;
const MAX_VELOC: f32 = 5.;
const MAX_NEIGHBORS: usize = 8;
const EDGE_TURN_MARGIN: f32 = 20.;

fn rand_vec2(rng: &mut ThreadRng, max_1: f32, max_2: f32, centered: bool) -> Vec2 {
    let mut x1 = rng.gen::<f32>();
    let mut x2 = rng.gen::<f32>();
    if centered {
        x1 -= 0.5;
        x2 -= 0.5;
    }
    let final1 = x1 * max_1;
    let final2 = x2 * max_2;
    Vec2::new(final1, final2)
    // Vec2::new(x1, x2)
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Boid {
    pos: Point2,
    vel: Vector2,
}

impl Boid {
    pub fn new(pos: Point2, vel: Vector2) -> Self {
        Self { pos, vel }
    }
    pub fn distance_to(&self, other: &Self) -> f32 {
        self.pos.distance(other.pos)
    }
    pub fn sq_distance_to(&self, other: &Self) -> f32 {
        self.pos.distance_squared(other.pos)
    }
    pub fn get_block(&self, block_size: &Vector2) -> usize {
        let block_xy = self.pos / *block_size;
        (block_xy[0] * block_xy[1]) as usize
    }
    pub fn limitSpeed(&mut self) {
        let norm = self.vel.length();
        // if norm > MAX_VELOC {
        // self.vel = self.vel / norm * MAX_VELOC;
        // }
        // else {
        //     self.vel = self.vel / norm * 0.5 * MAX_VELOC;
        // }
        self.vel = self.vel.normalize() * MAX_VELOC * 0.6;
    }
    pub fn updatePos(&mut self) {
        self.pos += self.vel
    }
    pub fn facingAngle(&self) -> f32 {
        let facing = self.vel[1].atan2(self.vel[0]) + 0.5 * std::f32::consts::PI;
        facing
    }
    pub fn get_drawparam(&self) -> DrawParam {
        graphics::DrawParam::new()
            .dest(self.pos)
            .rotation(self.facingAngle())
            .offset(Point2::new(0.5, 0.5))
            .scale(Vec2::new(0.2, 0.2))
    }
    pub fn draw(&self, assets: &mut Assets, ctx: &mut Context) {
        let drawparams = graphics::DrawParam::new()
            .dest(self.pos)
            .rotation(self.facingAngle())
            .offset(Point2::new(0.5, 0.5))
            .scale(Vec2::new(0.2, 0.2));
        graphics::draw(ctx, &assets.boid_image, drawparams);
    }
    pub fn keepWithinBounds(&mut self, width: f32, height: f32) {
        const margin: f32 = EDGE_TURN_MARGIN;
        const turnFactor: f32 = 1.;

        if (self.pos[0] < margin) {
            self.vel += Vector2::new(turnFactor, 0.);
        }
        if (self.pos[0] > width - margin) {
            self.vel -= Vector2::new(turnFactor, 0.);
        }
        if (self.pos[1] < margin) {
            self.vel += Vector2::new(0., turnFactor);
        }
        if (self.pos[1] > height - margin) {
            self.vel -= Vector2::new(0., turnFactor);
        }
    }
}

pub struct BoidCloud {
    pub width: f32,
    pub height: f32,
    pub boids: Vec<Boid>,
    pub boid_count: usize,
    //blocks_x: u8,
    //blocks_y: u8,
    //block_size: Vector2;
}

fn flyTowardsCenter(boid: &mut Boid, closest: &Vec<Boid>) {
    let mut neighbor_count: f32 = 0.;
    let mut center = Vector2::ZERO;
    for neighbor in closest {
        if boid.distance_to(neighbor) < VISUAL_RANGE {
            center += neighbor.pos;
            neighbor_count += 1.;
        }
    }
    if neighbor_count > 0. {
        center /= neighbor_count;
        boid.vel += (center - boid.pos) * COHERENCE
    };
}
fn avoidOthers(boid: &mut Boid, closest: &Vec<Boid>) {
    let mut delta = Vector2::new(0., 0.);
    for neighbor in closest {
        if boid.distance_to(neighbor) < AVOID_RANGE {
            delta += boid.pos - neighbor.pos;
        }
    }
    boid.vel += delta * SEPARATION;
}
fn matchVelocity(boid: &mut Boid, closest: &Vec<Boid>) {
    let mut neighbor_count: f32 = 0.;
    let mut avg_vel = Vector2::ZERO;
    for neighbor in closest {
        if boid.distance_to(neighbor) < VISUAL_RANGE {
            avg_vel += neighbor.vel;
            neighbor_count += 1.;
        }
    }
    if neighbor_count > 0. {
        avg_vel /= neighbor_count;
        boid.vel += (avg_vel - boid.vel) * ALIGNMENT;
    }
}

impl BoidCloud {
    pub fn new(
        boid_count: usize,
        width: f32,
        height: f32,
        rng: &mut ThreadRng,
        //blocks_x: u8,
        //blocks_y: u8
    ) -> GameResult<Self> {
        let boid_vec: Vec<Boid> = (0..boid_count)
            .map(|_| {
                Boid::new(
                    rand_vec2(rng, width, height, false),
                    rand_vec2(rng, MAX_VELOC / 2., MAX_VELOC / 2., true),
                )
            })
            .collect::<Vec<Boid>>();
        //let boids = vec![HashSet::new(); blocks_y * blocks_x];
        Ok(Self {
            width,
            height,
            // boids: vec![Boid::new(
            //     Point2::new(rng.rand_float() * (width-1.), rng.rand_float() * (height-1.)),
            //     Point2::new((rng.rand_float()-0.5) * 50., (rng.rand_float()-0.5) * 50.));
            //     boid_count],
            boids: boid_vec,
            // boids: (0..boid_count).map(|_| Boid::new(
            //     Point2::new(rng.rand_float() * (width-1.), rng.rand_float() * (height-1.)),
            //     Point2::new(rng.rand_float()-0.5, rng.rand_float()-0.5),
            // )).collect::<Vec<Boid>>(),
            boid_count
            //blocks_x,
            //blocks_y,
            //block_size: Vector2::new(width/(blocks_x as f32), height/(blocks_y as f32)),
        })
    }

    pub fn update(&mut self, width: f32, height: f32) {
        let mut boids = self.boids.clone();
        for mut boid in self.boids.iter_mut() {
            boids.sort_unstable_by(|a, b| {
                (&boid.distance_to(a))
                    .partial_cmp(&boid.sq_distance_to(b))
                    .unwrap()
            });
            if self.boid_count >= MAX_NEIGHBORS {
                let closest = boids[1..MAX_NEIGHBORS].to_vec();
                flyTowardsCenter(&mut boid, &closest);
                avoidOthers(&mut boid, &closest);
                matchVelocity(&mut boid, &closest);
            }
            boid.limitSpeed();
            boid.keepWithinBounds(width, height);
            boid.updatePos();
        }
    }
    // #[cfg(debug_assertions)]
    // pub fn draw(&self, assets: &mut Assets, ctx: &mut Context){
    //     for boid in self.boids.iter() {
    //         boid.draw(assets, ctx);
    //     }
    // }
    // #[cfg(not(debug_assertions))]
    pub fn add_boids_to_spritebatch(&self, img_batch: &mut SpriteBatch) {
        for boid in self.boids.iter() {
            img_batch.add(boid.get_drawparam());
        }
    }
}
