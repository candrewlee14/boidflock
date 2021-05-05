use ggez::conf;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, spritebatch::SpriteBatch, Color};
use ggez::{filesystem, Context, ContextBuilder, GameResult};
use glam::Vec2;
mod boidstuff;
use boidstuff::{boid::Boid, boid_cloud::BoidCloud};
mod assets;
use assets::Assets;
use rand::distributions::Standard;
use rand::prelude::*;

fn main() -> GameResult {
    // We add the CARGO_MANIFEST_DIR/resources to the resource paths
    // so that ggez will look in our cargo project directory for files.
    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        std::path::PathBuf::from("./resources")
    };
    let window_mode = conf::WindowMode::default()
        .fullscreen_type(conf::FullscreenType::Desktop)
        //.maximized(true)
        .borderless(true);
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .window_setup(conf::WindowSetup::default().title("Boid Flocking Simulation"))
        //.window_mode(conf::WindowMode::default().dimensions(1500.0, 1000.0))
        .window_mode(window_mode)
        .add_resource_path(resource_dir)
        .build()?;
    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut main_state = MainState::new(&mut ctx)?;

    // Run!
    event::run(&mut ctx, &mut event_loop, &mut main_state)
}

struct MainState {
    width: f32,
    height: f32,
    boid_cloud: BoidCloud,
    assets: Assets,
    //rng: Rand32,
    rng: ThreadRng,
    img_batch: SpriteBatch,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        //let mut rng = Rand32::new(u64::from_ne_bytes(seed));
        let mut rng = rand::thread_rng();
        // Load/create resources such as images here.
        let (width, height) = graphics::drawable_size(ctx);
        let boid_cloud = BoidCloud::new(500, width, height, &mut rng)?;
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
        // Update code here...
        self.boid_cloud
            .update(self.width, self.height, &mut self.rng);
        Ok(())
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
            _ => (), // Do nothing
        }
    }
    // #[cfg(debug_assertions)]
    // fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    //     graphics::clear(ctx, Color::new(0.,0.,0.,1.));
    //     // Draw code here...
    //     self.boid_cloud.draw(&mut self.assets, ctx);
    //     graphics::present(ctx)
    // }
    // #[cfg(not(debug_assertions))]
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::new(0., 0., 0., 1.));
        // Draw code here...
        self.img_batch.clear();
        self.boid_cloud
            .add_boids_to_spritebatch(&mut self.img_batch);
        graphics::draw(ctx, &self.img_batch, (Vec2::new(0.0, 0.0),))?;
        graphics::present(ctx)
    }
}
