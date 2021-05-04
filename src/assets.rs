use ggez::{
    graphics::{self, Color},
    Context, GameResult,
};

pub struct Assets {
    pub boid_image: graphics::Image,
}
impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        Ok(Assets {
            boid_image: graphics::Image::new(ctx, "/player.png")?,
        })
    }
}
