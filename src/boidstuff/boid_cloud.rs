use super::constants::*;
use crate::boidstuff::boid::Boid;
use crate::cliargs::BoidSimOpt;

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
}

pub struct BoidCloud {
    pub width: f32,
    pub height: f32,
    pub boids: Vec<Boid>,
    pub boid_count: usize,
    pub opt: BoidSimOpt,
}
impl BoidCloud {
    pub fn new(
        boid_count: usize,
        width: f32,
        height: f32,
        rng: &mut ThreadRng,
        opt: BoidSimOpt,
    ) -> GameResult<Self> {
        let boid_vec: Vec<Boid> = (0..boid_count)
            .map(|_| {
                Boid::new(
                    rand_vec2(rng, width, height, false),
                    rand_vec2(rng, opt.MAX_VELOC / 2., opt.MAX_VELOC / 2., true),
                )
            })
            .collect::<Vec<Boid>>();
        Ok(Self {
            width,
            height,
            boids: boid_vec,
            boid_count,
            opt,
        })
    }

    pub fn update(&mut self, width: f32, height: f32, rng: &mut ThreadRng) {
        let boids = self.boids.clone();
        let opt = self.opt.clone();
        for mut boid in self.boids.iter_mut() {
            if self.boid_count >= self.opt.MAX_NEIGHBORS {
                let mut dist_and_closest = boids
                    .iter()
                    .filter_map(|other| {
                        if let Some(dist) = boid.get_dist_if_in_sight(other, &opt) {
                            return Some((dist, other.clone()));
                        }
                        None
                    })
                    .take(self.opt.NEIGHBORS_TO_SEE)
                    .collect::<Vec<(f32, Boid)>>();
                if dist_and_closest.len() > self.opt.MAX_NEIGHBORS {
                    dist_and_closest.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                }
                let closest: Vec<Boid> = dist_and_closest
                    .iter()
                    .take(self.opt.MAX_NEIGHBORS)
                    .map(|tup| tup.1)
                    .collect();
                if closest.len() > 0 {
                    boid.min_dist = boid.distance_to(&closest[0]);
                    boid.fly_towards_center(&closest, &self.opt);
                    boid.avoid_other_boids(&closest, &self.opt);
                    boid.match_velocities(&closest, &self.opt);
                }
            }
            boid.random_vel_change(rng, &self.opt);
            boid.limit_speed(&self.opt);
            boid.keep_within_bounds(width, height, &self.opt);
            boid.update_pos();
        }
    }

    pub fn add_boids_to_spritebatch(&self, img_batch: &mut SpriteBatch) {
        for boid in self.boids.iter() {
            img_batch.add(boid.get_drawparam(&self.opt));
        }
    }
}
