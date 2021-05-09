
use rand::prelude::*;
use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

use structopt::StructOpt;
use rand_xoshiro::Xoroshiro128Plus;
use boids::cliargs::BoidSimOpt;
use boids::boid_cloud::BoidCloud;

fn boid_sim(c: &mut Criterion) {
    let opt = BoidSimOpt::from_iter(vec![""]);
    let mut rng = Xoroshiro128Plus::seed_from_u64(12345);
    let mut boid_cloud = BoidCloud::new(2000, 2000., 1000., &mut rng, opt);

    c.bench_function("boid simulation updates", 
    |b| {
        b.iter(|| boid_cloud.update(&mut rng));
    });
}

criterion_group!(benches, boid_sim);
criterion_main!(benches);