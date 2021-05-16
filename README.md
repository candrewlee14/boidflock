# Boidflock

This is a Rust implementation of a boid flocking simulation using the ggez graphics crate.

The CLI for this program is built using the structopt crate.

## Examples

`cargo run --release`
<img src="./default.gif">


`cargo run --release -- --zoom-scale 0.7 --visual-range 150 --avoid-range 30 --sight-angle 1. --coherence .7`
<img src="./example1.gif">

`cargo run --release -- --boid-count 12000 --zoom-scale .35 --visual-range 120 --sight-angle 1.5 --avoid-range 30 --alignment .95 --coherence .45`
<img src="./example2.gif">


## Configuration

There are lots of options to change the simulation from its default settings. Here's an example configuration for zooming out with many boids:
`cargo run --release -- --boid-count 15000 --zoom-scale .3 --visual-range 120 --sight-angle 1.5 --avoid-range 25`.

Running `cargo run --release -- --help` yields a screen with the options shown below:
    
    Boid Flocking Simulation
    Andrew Lee
    A flocking simulation built in Rust with ggez

    USAGE:
        boidflock.exe [OPTIONS]

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
            --seed <seed>
                Seed to determine initial positions and random rotations applied to boids. This make the simulation
                deterministic. If set to default 0, the seed will be randomly generated : u64 [default: 0]
            --boid-count <boid-count>                            Count of boids to simulate : [0, INF) [default: 2000]
            --coherence <coherence>
                Coefficient for boid aiming for center of local neighbor mass : [0, 1] [default: 0.035]

            --separation <separation>
                Coefficient for boid avoidance within AVOID_RANGE : [0, 1] [default: 0.25]

            --alignment <alignment>
                Coefficient for boid alignment of velocity within SIGHT_RANGE : [0, 1] [default: 0.75]

            --avoid-range <avoid-range>
                Pixel distance for a boid to avoid others in line of sight : [0, INF) [default: 21.]

            --visual-range <visual-range>
                Pixel sight distance for each boid : [0, INF) [default: 80.]

            --sight-angle <sight-angle>
                Sight pie-slice angle for each boid in radians : [0, 2*PI] [default: 2.3]

            --sight-samples <sight-samples>
                Samples to take get cells within sight angle range : [1, INF) [default: 3]

            --max-rand-rotate <max-rand-rotate>
                Max random velocity rotation angle for boids : [0, 2*PI] [default: 0.3]

            --min-veloc <min-veloc>
                Minimum boid velocity in pixels : [0, MAX_VELOC] [default: 3.]

            --max-veloc <max-veloc>
                Maximum boid velocity in pixels : [MIN_VELOC, INF) [default: 5.]

            --cur-cell-neighbors <cur-cell-neighbors>
                Maximum number of boid neighbors within current sight range cell to consider for calculations : [0,
                BOID_COUNT] [default: 30]
            --forward-cell-neighbors <forward-cell-neighbors>
                Number of boids in sight range in sight range cell ahead to consider : [0, BOID_COUNT] [default: 10]

            --edge-turn-margin <edge-turn-margin>
                Pixel distance from screen edge for boids to turn away from to stay on screen : [0, INF) [default: 25.]

            --edge-turn-factor <edge-turn-factor>
                Coefficient to turn away from screen edges : [0, 1] [default: 0.2]

            --img-scale <img-scale>                              Scale for boid image : [0, INF) [default: 0.32]
            --zoom-scale <zoom-scale>
                Scale to zoom. Above 1 zooms in, below 1 down to 0 zooms out : [0, INF) [default: 0.8]

## Information

This has been awesome project for me for studying complex behavior generated out of simple rules.

I learned lots about practical applications of algorithmic complexity and optimization tradeoffs, and how to use a graphics library.

Enjoy the show!
