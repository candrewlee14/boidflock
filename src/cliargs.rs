use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone, Default)]
#[structopt(
    name = "Boid Flocking Simulation",
    about = "A flocking simulation built in Rust with ggez",
    author = "Andrew Lee",
    no_version,
    global_settings = &[structopt::clap::AppSettings::ColoredHelp, structopt::clap::AppSettings::DeriveDisplayOrder]
)]
#[allow(non_snake_case)]
pub struct BoidSimOpt {
    /// Count of boids to simulate : [0, INF)
    #[structopt(long, default_value = "2000")]
    pub BOID_COUNT: usize,

    /// Coefficient for boid aiming for center of local neighbor mass : [0, 1]
    #[structopt(long, default_value = "0.035")]
    pub COHERENCE: f32,

    /// Coefficient for boid avoidance within AVOID_RANGE : [0, 1]
    #[structopt(long, default_value = "0.25")]
    pub SEPARATION: f32,

    /// Coefficient for boid alignment of velocity within SIGHT_RANGE : [0, 1]
    #[structopt(long, default_value = "0.75")]
    pub ALIGNMENT: f32,

    /// Pixel distance for a boid to avoid others in line of sight : [0, INF)
    #[structopt(long, default_value = "21.")]
    pub AVOID_RANGE: f32,

    /// Pixel sight distance for each boid : [0, INF)
    #[structopt(long, default_value = "80.")]
    pub VISUAL_RANGE: f32,

    /// Sight pie-slice angle for each boid in radians : [0, 2*PI]
    #[structopt(long, default_value = "2.3")]
    pub SIGHT_ANGLE: f32,

    /// Samples to take get cells within sight angle range : [1, INF)
    #[structopt(long, default_value = "3")]
    pub SIGHT_SAMPLES: usize,

    /// Max random velocity rotation angle for boids : [0, 2*PI]
    #[structopt(long, default_value = "0.3")]
    pub MAX_RAND_ROTATE: f32,

    /// Minimum boid velocity in pixels : [0, MAX_VELOC]
    #[structopt(long, default_value = "3.")]
    pub MIN_VELOC: f32,

    /// Maximum boid velocity in pixels : [MIN_VELOC, INF)
    #[structopt(long, default_value = "5.")]
    pub MAX_VELOC: f32,

    /// Maximum number of boid neighbors within current sight range cell
    /// to consider for calculations : [0, BOID_COUNT]
    #[structopt(long, default_value = "30")]
    pub CUR_CELL_NEIGHBORS: usize,

    /// Number of boids in sight range in sight range cell ahead to consider : [0, BOID_COUNT]
    #[structopt(long, default_value = "10")]
    pub FORWARD_CELL_NEIGHBORS: usize,

    /// Pixel distance from screen edge for boids to turn away from
    /// to stay on screen : [0, INF)
    #[structopt(long, default_value = "25.")]
    pub EDGE_TURN_MARGIN: f32,

    /// Coefficient to turn away from screen edges : [0, 1]
    #[structopt(long, default_value = "0.2")]
    pub EDGE_TURN_FACTOR: f32,

    /// Scale for boid image : [0, INF)
    #[structopt(long, default_value = "0.32")]
    pub IMG_SCALE: f32,

    /// Scale to zoom. Above 1 zooms in, below 1 down to 0 zooms out : [0, INF)
    #[structopt(long, default_value = "0.8")]
    pub ZOOM_SCALE: f32,
}
