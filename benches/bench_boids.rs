use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use rand::prelude::*;

use boids::boid_cloud::BoidCloud;
use boids::cliargs::BoidSimOpt;
use rand_xoshiro::Xoroshiro128Plus;
use structopt::StructOpt;

fn boid_sim(c: &mut Criterion) {
    let opt = BoidSimOpt::from_iter(vec![""]);
    let mut rng = Xoroshiro128Plus::seed_from_u64(12345);
    let mut boid_cloud = BoidCloud::new(2000, 2000., 1000., &mut rng, opt);

    c.bench_function("boid simulation updates", |b| {
        b.iter(|| boid_cloud.update(&mut rng));
    });
}

criterion_group!(benches, boid_sim);
criterion_main!(benches);
