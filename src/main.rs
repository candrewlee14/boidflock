use ggez::conf;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, spritebatch::SpriteBatch, Color};
use ggez::{Context, ContextBuilder, GameResult};
use glam::Vec2;
use structopt::StructOpt;

use rand::prelude::*;
use rand_xoshiro::Xoroshiro128Plus;

mod assets;
mod boid;
mod boid_cloud;
mod cliargs;

use assets::Assets;
use boid_cloud::BoidCloud;
use cliargs::BoidSimOpt;

fn main() -> GameResult {
    let opt = BoidSimOpt::from_args();
    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        std::path::PathBuf::from("./resources")
    };
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .window_setup(
            conf::WindowSetup::default()
                .title("Boid Flocking Simulation")
                .icon("/player.png"),
        )
        .window_mode(
            conf::WindowMode::default()
                .dimensions(1920.0, 1080.0)
                .resizable(true)
                .maximized(true),
        )
        .add_resource_path(resource_dir)
        .build()?;
    let mut main_state = MainState::new(&mut ctx, opt)?;

    // Run!
    event::run(&mut ctx, &mut event_loop, &mut main_state)
}

struct MainState {
    width: f32,
    height: f32,
    boid_cloud: BoidCloud,
    assets: Assets,
    rng: Xoroshiro128Plus,
    img_batch: SpriteBatch,
}

impl MainState {
    pub fn new(ctx: &mut Context, opt: BoidSimOpt) -> GameResult<Self> {
        let mut rng = Xoroshiro128Plus::from_entropy();
        let (mut width, mut height) = graphics::drawable_size(ctx);
        width /= opt.ZOOM_SCALE;
        height /= opt.ZOOM_SCALE;
        let boid_cloud = BoidCloud::new(opt.BOID_COUNT, width, height, &mut rng, opt);
        let assets = Assets::new(ctx)?;
        let img_batch = SpriteBatch::new(assets.boid_image.clone());
        Ok(Self {
            width,
            height,
            boid_cloud,
            assets,
            rng,
            img_batch,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // let _boids = self.boid_cloud.boids.clone();
        // let _optclone = self.boid_cloud.opt.clone();
        self.boid_cloud.update(&mut self.rng);
        Ok(())
    }

    fn resize_event(&mut self, _ctx: &mut Context, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: event::KeyCode,
        _keymod: event::KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            event::KeyCode::Escape | event::KeyCode::Q => event::quit(ctx),
            _ => (),
        }
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::new(0., 0., 0., 1.));
        self.img_batch.clear();
        self.boid_cloud
            .add_boids_to_spritebatch(&mut self.img_batch);
        graphics::draw(ctx, &self.img_batch, (Vec2::new(0.0, 0.0),))?;
        graphics::present(ctx)
    }
}
