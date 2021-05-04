use super::constants::*;
use crate::boidstuff::boid::Boid;

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

pub struct BoidCloud {
    pub width: f32,
    pub height: f32,
    pub boids: Vec<Boid>,
    pub boid_count: usize,
}

fn fly_towards_center(boid: &mut Boid, closest: &Vec<Boid>) {
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
fn avoid_other_boids(boid: &mut Boid, closest: &Vec<Boid>) {
    let mut delta = Vector2::new(0., 0.);
    for neighbor in closest {
        if boid.distance_to(neighbor) < AVOID_RANGE {
            delta += boid.pos - neighbor.pos;
        }
    }
    boid.vel += delta * SEPARATION;
}
fn match_velocities(boid: &mut Boid, closest: &Vec<Boid>) {
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
    ) -> GameResult<Self> {
        let boid_vec: Vec<Boid> = (0..boid_count)
            .map(|_| {
                Boid::new(
                    rand_vec2(rng, width, height, false),
                    rand_vec2(rng, MAX_VELOC / 2., MAX_VELOC / 2., true),
                )
            })
            .collect::<Vec<Boid>>();
        Ok(Self {
            width,
            height,
            boids: boid_vec,
            boid_count,
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
                boid.min_dist = boid.distance_to(&closest[0]);
                fly_towards_center(&mut boid, &closest);
                avoid_other_boids(&mut boid, &closest);
                match_velocities(&mut boid, &closest);
            }            
            boid.limit_speed();
            boid.keep_within_bounds(width, height);
            boid.update_pos();
        }
    }

    pub fn add_boids_to_spritebatch(&self, img_batch: &mut SpriteBatch) {
        for boid in self.boids.iter() {
            img_batch.add(boid.get_drawparam());
        }
    }
}
