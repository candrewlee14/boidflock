# Boidflock

This is a Rust implementation of a boid flocking simulation using the ggez graphics crate.

The CLI for this program is built using the structopt crate.

<img src="https://repository-images.githubusercontent.com/364743636/4c202000-b535-11eb-8d51-20fd3b01c88f" width="700px">

Running `cargo run --release -- --help` yields a screen with the options below:
    
    Boid Flocking Simulation
    Andrew Lee
    A flocking simulation built in Rust with ggez

    USAGE:
        boids.exe [OPTIONS]

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
            --boid-count <boid-count>                Count of boids to simulate : [0, INF) [default: 2000]
            --coherence <coherence>
                Coefficient for boid aiming for center of local neighbor mass : [0, 1] [default: 0.035]

            --separation <separation>
                Coefficient for boid avoidance within AVOID_RANGE : [0, 1] [default: 0.25]

            --alignment <alignment>
                Coefficient for boid alignment of velocity within SIGHT_RANGE : [0, 1] [default: 0.75]

            --avoid-range <avoid-range>
                Pixel distance for a boid to avoid others in line of sight : [0, INF) [default: 21.]

            --visual-range <visual-range>            Pixel sight distance for each boid : [0, INF) [default: 60.]
            --sight-angle <sight-angle>
                Sight pie-slice angle for each boid in radians : [0, 2*PI] [default: 2.3]

            --max-rand-rotate <max-rand-rotate>      Max random velocity rotation angle for boids : [0, 2*PI] [default: 0.3]
            --min-veloc <min-veloc>                  Minimum boid velocity in pixels : [0, MAX_VELOC] [default: 3.]
            --max-veloc <max-veloc>                  Maximum boid velocity in pixels : [MIN_VELOC, INF) [default: 5.]
            --max-neighbors <max-neighbors>
                Maximum number of boid neighbors within sight range to consider for calculations : [0, BOID_COUNT] [default:
                10]
            --neighbors-to-see <neighbors-to-see>
                Number of boids in sight range to consider and sort, of which the nearest MAX_NEIGHBORS boids are taken :
                [0, BOID_COUNT] [default: 15]
            --edge-turn-margin <edge-turn-margin>
                Pixel distance from screen edge for boids to turn away from to stay on screen : [0, INF) [default: 20.]

            --edge-turn-factor <edge-turn-factor>    Coefficient to turn away from screen edges : [0, 1] [default: 0.2]
            --img-scale <img-scale>                  Scale for boid image : [0, INF) [default: 0.32]
            --zoom-scale <zoom-scale>
                Scale to zoom. Above 1 zooms in, below 1 down to 0 zooms out : [0, INF) [default: 1.]

## Information

This has been awesome project for me for studying complex behavior generated out of simple rules.

I learned lots about practical applications of algorithmic complexity and optimization tradeoffs, and how to use a graphics library.

Enjoy the show!
